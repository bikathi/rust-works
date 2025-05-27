use std::{fs::read_dir, path::{Path, PathBuf}};
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
        pattern: &Regex,
        recursive: bool
    ) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut discovered_entries: Vec<PathBuf> = Vec::new(); 
        
        for entry in folder_path.read_dir()? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            
            if file_type.is_dir() {
                match FileUtils::get_base_folder_children(&entry.path(), pattern, recursive) {
                    Ok(children) => discovered_entries.extend_from_slice(&children),
                    Err(_) => ()
                }
            }
            
            if file_type.is_file() & pattern.is_match(entry.path().to_str().unwrap()) {
                discovered_entries.push(entry.path());
            }
        }
        
        // sort entries by their path as order is not guaranteed
        discovered_entries.sort();
        
        Ok(discovered_entries)
    }
}