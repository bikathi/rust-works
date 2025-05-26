use clap::Parser;
use cn::{
    file_utils::FileUtils, 
    input_handler::{CliInput, ModeCommands}
};

fn main() {
    let cli_args:CliInput = CliInput::parse();
    
    match cli_args.get_mode_commands() {
        ModeCommands::Single { file_name, new_name } => {
            FileUtils::rename_file(file_name, new_name);
        },
        ModeCommands::Bulk { directory, pattern, replacement, recursive, rename_folders, rename_files, no_warnings } => {
            // // This block will execute when the 'bulk' subcommand is used.
            // println!("'bulk' subcommand activated:");
            // println!("  Directory: {}", directory.display());
            // println!("  Pattern: '{}'", pattern);
            // println!("  Replacement: '{}'", replacement);
            // println!("  Recursive: {}", recursive);
            // println!("  Rename Folders: {}", rename_folders);
            // println!("  Rename Files: {}", rename_files);
            // println!("  Display Warning: {}", no_warnings);
            // 
            
            if let true = directory.exists() {
                println!("folder exists!");
            } else {
                eprintln!("folder does not exist!");
            }
        }
    }
}
