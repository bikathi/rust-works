use std::path::{Path, PathBuf};

pub struct FileUtils;

impl FileUtils {
    pub fn rename_file(file_path: &PathBuf, new_name: &str) {
        match std::fs::rename(file_path, new_name) {
            Ok(_) => println!("file renamed"),
            Err(e) => eprintln!("Failed on error {}", e.kind()),
        };
    }
    
    pub fn get_base_folder_children(folder_path: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        let entries = folder_path.read_dir()?;
        let files: Vec<PathBuf> = entries
            .filter_map(|entry_result| {
                if let Ok(e) = entry_result {
                    if let Ok(file_type) = e.file_type() {
                        if file_type.is_file() {
                            return Some(e.path())
                        }
                    }
                }
                
                None
            })
        .collect();
        
        Ok(files)
    }
}