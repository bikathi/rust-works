pub struct FileUtils;

impl FileUtils {
    pub fn rename_file(file_path: &String, new_name: &str) {
        match std::fs::rename(file_path, new_name) {
            Ok(_) => println!("file renamed"),
            Err(e) => eprintln!("Failed on error {}", e.kind()),
        };
    }  
}