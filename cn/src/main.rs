use std::{path::PathBuf};

use clap::Parser;
use cn::{
    file_utils::FileUtils, 
    input_handler::{CliInput, ModeCommands, get_user_consent}
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
            
            if let Ok(children) = FileUtils::get_base_folder_children(&directory, &regex, *recursive, *include_folders) {
                if children.is_empty() {
                    panic!("Found no entries to work on!");
                }
                
                // get the proposed changes
                let proposed_changes: Vec<(PathBuf, PathBuf)> = FileUtils::derive_new_names(&pattern, &children, replacement);
                
                // present this info to the user
                println!("Discovered {} matching entires", children.len());
                proposed_changes
                    .iter()
                    .for_each(|(old_path, new_path)| {
                        if let (pb, Some(new_os_str)) = (old_path, new_path.file_name()) {
                            // the unwrap() calls here is because logic shouldn't fail at this point
                            println!("{} [{:?}] => {new_os_str:?}", pb.to_str().unwrap(), pb.file_name().unwrap());
                        }
                    });
                
                // ask for consent if they didn't disable the warning.
                // Highly discourage end user from disabling the warnings
                if *no_warnings {
                    FileUtils::execute_rename(proposed_changes);
                } else {
                    if let Ok(proceeed_to_apply) = get_user_consent() {
                        if proceeed_to_apply {
                            FileUtils::execute_rename(proposed_changes);
                            println!("Done!");
                        } else {
                            println!("Exiting without change!");
                        } 
                        
                        return;
                    }
                }
            }
        }
    }
}
