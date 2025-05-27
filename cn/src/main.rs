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
        ModeCommands::Bulk { directory, pattern, replacement, recursive, include_folders, no_warnings } => {
            let regex = Regex::new(pattern).unwrap_or_else(|_| {panic!("Invalid pattern!")});
            
            if !directory.exists() {
                panic!("Base directory provided does not exist!");
            }
            
            if !directory.is_dir() {
                panic!("Base directory must be a folder!");
            }
            
            if let Ok(children) = FileUtils::get_base_folder_children(
                &directory, 
                &regex, 
                *recursive,
                *include_folders
            ) {
                if children.is_empty() {
                    panic!("Found no files to work on!");
                }
                
                // present this info to the user
                if !*no_warnings {
                    println!(
                        "Discovered {} file{}:", 
                        children.len(),
                        if *include_folders { "s and/or folders" } else { "s" }
                    );
                    children.into_iter().for_each(|c| println!("{c:?}"));
                }
            }
        }
    }
}
