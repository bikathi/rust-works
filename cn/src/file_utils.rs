use std::path::PathBuf;

pub struct FileUtils;

impl FileUtils {
    pub fn rename_file(file_path: &PathBuf, new_name: &str) {
        match std::fs::rename(file_path, new_name) {
            Ok(_) => println!("file renamed"),
            Err(e) => eprintln!("Failed on error {}", e.kind()),
        };
    } 
   
    pub fn folder_exists(folder_path: &PathBuf) -> bool {
        match std::fs::exists(folder_path) {
           Ok(_) => true,
           _ => false
        }
    } 
}