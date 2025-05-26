use clap::Parser;
use cn::{
    file_utils::FileUtils, 
    input_handler::{CliInput, ModeCommands}
};
use regex::Regex;

fn main() {
    let cli_args:CliInput = CliInput::parse();
    
    match cli_args.get_mode_commands() {
        ModeCommands::Single { file_name, new_name } => {
            FileUtils::rename_file(file_name, new_name);
        },
        ModeCommands::Bulk { directory, pattern, replacement, recursive, rename_folders, rename_files, no_warnings } => {
            let regex = Regex::new(pattern).unwrap_or_else(|_| {panic!("Invalid pattern!")});
            
            if !directory.exists() {
                panic!("--directory provided does not exist!");
            }
            
            if !directory.is_dir() {
                panic!("'--directory' must be a folder!");
            }
            
            if let Ok(children) = FileUtils::get_base_folder_children(&directory, &regex) {
                if children.is_empty() {
                    eprintln!("Cannot work on empty folder!");
                }
                
                println!("Folder's children files: {:?}", children);
            }
        }
    }
}
