/* This file adds tags line to your file based on its title. Intended for use with Obsidian.md.

For example:

The file `'lang.rust.data.type.String'`
would have its last line changed to
`'#flashcard #lang/rust/data/type/String'`

Creates temporary files before writing them to their target destinations.
Assumes all filenames and contents are UTF-8. */

use std::ffi::OsString;
use std::fs;
use std::fs::{create_dir, File, remove_dir_all};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

static TARGET_DIR: &str = "../../faust/";
static TEMP_DIR: &str = "temp/";

fn main() {
    // Get note-file names (i.e. that are .md).
    let filenames = fs::read_dir(TARGET_DIR)
        .unwrap()
        // Turn each item into a PathBuf.
        .map(|filename|
            PathBuf::from( &filename
                .unwrap()
                .path()
            )
        )
        // Filter out non-markdown files.
        .filter(|path|
            path.extension() == Some(&OsString::from("md"))
        )
        // Filter out files without '.'.
        .filter(|path|
            path.file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                .contains('.')
        )
        // Filter out lib-specific implementations (using ~).
        .filter(|path|
            !String::from(path.to_str().unwrap()).contains('~')
        )
        // Filter out files with no lines.
        .filter(| path| {
            let reader = BufReader::new(File::open(&path).unwrap());
            let lines = reader
                .lines()
                .collect::<Vec<_>>();
            lines.len() > 0
        }

        )
        .collect::<Vec<_>>();

    // Create temporary dir to write files without overwriting original files.
    if Path::new(TEMP_DIR).exists() {
        println!("Cannot create temp dir because it exists already. Exiting without changes.");
        exit(1);
    }
    create_dir(TEMP_DIR).unwrap();

    // Iterate through files to add tags.
    for filepath in &filenames {

        let file = File::open(&filepath).unwrap();
        let reader = BufReader::new(file);

        let mut lines = reader
            .lines()
            .map(|line| line.unwrap())
            .collect::<Vec<_>>();

        let tags_row = lines.len().saturating_sub(1);
        let tags_indicator = "TAGS";

        let filename = &filepath
            .file_name()
            .unwrap()
            .to_str()
            .map(|s| s.to_string())
            .unwrap();

        let filestem = &filepath
            .file_stem()
            .unwrap()
            .to_str()
            .map(|s| s.to_string())
            .unwrap();

        // Write tags.
        let new_tag = filestem.replace('.', "/");
        let new_tags_line =
            String::from(tags_indicator) + ": "
            + "#flashcard "
            + "#" + &new_tag;
        lines[tags_row] = new_tags_line;

        // Write to file in temp dir.
        let new_filepath = PathBuf::from(String::from(TEMP_DIR) + &filename);
        let mut new_file = File::create(new_filepath).unwrap();
        write!(new_file, "{}", lines.join("\n")).unwrap();
    }

    // Move temp files to destination dir.
    for destination in &filenames {
        let temp_filepath = PathBuf::from(
            String::from(TEMP_DIR)
            + destination
                .file_name()
                .unwrap()
                .to_str()
                .unwrap());
        eprintln!("temp_filepath = {:?}", temp_filepath);
        eprintln!("destination = {:?}", destination);
        fs::rename(temp_filepath, destination).unwrap();
    }

    // Remove temporary dir.
    remove_dir_all(TEMP_DIR).unwrap();
    if Path::new(TEMP_DIR).exists() {
        println!("Warning: temp dir could not be deleted.");
        exit(1);
    }

    println!("Tagging safely completed!");
}
