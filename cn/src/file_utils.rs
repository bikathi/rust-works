use crate::input_handler::{DisplayMode, display_proposed_changes, print_or_else};
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::{
    borrow::Cow,
    io::Write,
    path::{Path, PathBuf},
};

pub struct FileUtils;

impl FileUtils {
    pub fn rename_file(file_path: &PathBuf, new_name: &str) {
        match std::fs::rename(file_path, new_name) {
            Ok(_) => println!("file renamed"),
            Err(e) => eprintln!("Rename failed on error {}", e.kind()),
        };
    }

    pub fn get_base_folder_children(
        folder_path: &Path,
        matching_pattern: &Regex,
        recursive: bool,
        include_folders: bool,
    ) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut discovered_entries: Vec<PathBuf> = Vec::new();

        for entry in folder_path.read_dir()? {
            let entry = entry?;
            let file_type = entry.file_type()?;

            if file_type.is_dir() {
                if include_folders & matching_pattern.is_match(entry.path().to_str().unwrap()) {
                    discovered_entries.push(entry.path());
                }

                if recursive {
                    match FileUtils::get_base_folder_children(
                        &entry.path(),
                        matching_pattern,
                        recursive,
                        include_folders,
                    ) {
                        Ok(children) => discovered_entries.extend_from_slice(&children),
                        Err(_) => (),
                    }
                }
            }

            if file_type.is_file() & matching_pattern.is_match(entry.path().to_str().unwrap()) {
                discovered_entries.push(entry.path());
            }
        }

        // sort entries by their path as order is not guaranteed
        discovered_entries.sort();

        Ok(discovered_entries)
    }

    pub fn derive_new_names(
        matching_pattern: Regex,
        old_path: Vec<PathBuf>,
        replacement_pattern: &str,
    ) -> Vec<(PathBuf, PathBuf)> {
        // the response Vec
        let mut changes_pair: Vec<(PathBuf, PathBuf)> = Vec::new();

        for path in old_path {
            // Attempt to get the original name as a favourable string type
            let original_name_os_str = match path.file_name() {
                Some(name) => name,
                None => {
                    eprintln!(
                        "Warning. Could not get file/folder name for path: {}",
                        path.display()
                    );
                    continue; // We skip this folder/file path
                }
            };

            // Convert original name to Cow<'_, str> and apply the Regex pattern using replace_all to
            // get the base for the new name
            let original_name: Cow<'_, str> = original_name_os_str.to_string_lossy();
            let new_name_cow: Cow<'_, str> =
                Regex::replace_all(&matching_pattern, &original_name, replacement_pattern);

            let mut new_name_string: String = new_name_cow.to_string();

            // Get the parent of the item being renamed and attempt to combine it with the old name
            let parent_dir: &Path = path.parent().unwrap_or_else(|| Path::new(".")); // Assume item is in the CWD
            let mut current_new_path: PathBuf = parent_dir.join(&new_name_string);

            // Handling for name collisions
            // Append a counter e.g. my_txt_file_1.txt if new path already exists and is not the same as the
            // original path (meaning no change was made)
            let mut counter: i32 = 1;
            while current_new_path.exists() && current_new_path != *path {
                let (stem, extension) = (
                    current_new_path.file_stem().map(|s| s.to_string_lossy()),
                    current_new_path.extension().map(|e| e.to_string_lossy()),
                );

                new_name_string = format!(
                    "{}_{}{}",
                    stem.unwrap_or_default(),
                    counter,
                    extension.map_or("".into(), |ext| format!(".{}", ext))
                );
                current_new_path = parent_dir.join(&new_name_string);
                counter += 1;
            }

            // display proposed changes to the user
            display_proposed_changes((&path, &current_new_path));

            changes_pair.push((path.to_owned(), current_new_path));
        }

        changes_pair
    }

    pub fn execute_rename(update_pair: &Vec<(PathBuf, PathBuf)>) {
        for (old_path, new_path) in update_pair {
            if let Err(e) = std::fs::rename(&old_path, &new_path) {
                eprintln!(
                    "Failed to rename {} to {}! Cause: {}",
                    old_path.display(),
                    new_path.display(),
                    e
                );
            }
        }
    }
}

pub fn log_executed_changes(change_pair: Vec<(PathBuf, PathBuf)>, file_location: PathBuf) {
    // this function logs changes to <file_location>
    let mut output_file: File;

    if let Ok(_) = file_location.try_exists() {
        // open the file in write mode
        output_file = File::create(file_location).unwrap();
    } else {
        // else create a new file
        output_file = File::create_new(file_location).unwrap();
    }

    for pair in change_pair {
        let mut formatted_string: String =
            print_or_else((&pair.0, &pair.1), DisplayMode::FORMAT).unwrap();
        formatted_string.push_str("\n");
        output_file.write(formatted_string.as_bytes()).unwrap();
    }
}

pub fn parse_logs_to_buffer(log_file: &PathBuf) -> Result<Vec<String>, std::io::Error> {
    match log_file.try_exists() {
        Ok(_) => {
            let logs = std::fs::read_to_string(log_file)?;
            Ok(logs.lines().map(|l| l.to_string()).collect())
        }
        Err(e) => Err(e),
    }
}
