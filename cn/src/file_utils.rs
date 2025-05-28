use std::{borrow::Cow, path::{Path, PathBuf}};
use regex::Regex;

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
        include_folders: bool
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
                    match FileUtils::get_base_folder_children(&entry.path(), matching_pattern, recursive, include_folders) {
                        Ok(children) => discovered_entries.extend_from_slice(&children),
                        Err(_) => ()
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
    
    pub fn derive_new_names(selector_pattern: &str, old_path: &Vec<PathBuf>, replacement_pattern: &str) -> Vec<(PathBuf, PathBuf)> {
        // the response Vec
        let mut changes_pair: Vec<(PathBuf, PathBuf)> = Vec::new(); 
        
        // create a Regex pattern out of the replacement_pattern. Can be capture groups!
        let regex = Regex::new(replacement_pattern).unwrap_or_else(|_| {panic!("Invalid pattern!")});
        
        for path in old_path {
            // Attempt to get the original name as a favourable string type
            let original_name_os_str = match path.file_name() {
                Some(name) => name,
                None => {
                    eprintln!("Warning. Could not get file/folder name for path: {}", path.display());
                    continue; // We skip this folder/file path
                },
            };
            
            // Convert original name to Cow<'_, str> and apply the Regex pattern using replace_all to
            // Get the base for the new name
            let original_name: Cow<'_, str> = original_name_os_str.to_string_lossy();
            let new_name_cow: Cow<'_, str> = Regex::replace_all(&regex, &original_name, replacement_pattern);
            
            let mut new_name_string: String = new_name_cow.to_string(); // convert name to an owned string
            println!("selector pattern: {}. orginal name: {}. regex: {} from replacement pattern: {}. new name: {}", selector_pattern, &original_name, regex.as_str(), &replacement_pattern, &new_name_string);
            
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
                    current_new_path.extension().map(|e| e.to_string_lossy())
                );
                
                // Recreate a new name with counter, preserving extension if present
                new_name_string = format!(
                    "{}_{}{}",
                    stem.unwrap_or_default(),
                    counter,
                    extension.map_or("".into(), |ext| format!(".{}", ext))
                );
                current_new_path = parent_dir.join(&new_name_string);
                counter += 1;
            }
            
            changes_pair.push((path.clone(), current_new_path.clone()));
        }
        
        changes_pair
    }
    
    pub fn execute_rename(update_pair: Vec<(PathBuf, PathBuf)>) {
        for (old_path, new_path) in update_pair {
            if let Err(e) = std::fs::rename(&old_path, &new_path) {
                eprintln!("Failed to rename {} to {}! Cause: {}", old_path.display(), new_path.display(), e);
            }
        }
    }
}