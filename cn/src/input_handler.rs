use clap::{Parser,Subcommand};
use std::{path::PathBuf}; // a special string used for handling file paths

#[derive(PartialEq, Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliInput {
    #[command(subcommand)] // This macro tells clap to look for subcommands
    command: ModeCommands,
}

impl CliInput {
    pub fn get_mode_commands(self: &Self) -> &ModeCommands {
        &self.command
    }
}

// the available sub-commands for the renaming tool
#[derive(Subcommand, Debug, PartialEq, Clone)]
pub enum ModeCommands {
    // when doing a simple rename to a single file
    Single {
        #[arg(short = 'f', long = "file-name")]
        file_name: PathBuf,
        
        #[arg(short = 'n', long = "new-name")]
        new_name: String,
    },
    
    // Bulk renames files and/or folders in a specified directory based on a regex pattern.
    Bulk {
        /// The base directory in which to perform the bulk renaming.
        #[arg(short, long, value_name = "DIRECTORY_PATH")]
        directory: PathBuf,
        
        /// The regular expression pattern to match against file/folder names in this base directory.
        #[arg(short, long, value_name = "REGEX_PATTERN")]
        pattern: String,
        
        /// The replacement string for matched patterns. Capture groups can be used (e.g., '$1').
        #[arg(long, value_name = "REPLACEMENT_STRING")]
        replacement: String,
        
        /// If set, the tool will recursively traverse subdirectories.
        #[arg(long)]
        recursive: bool,
        
        /// If set, folders will also be renamed based on the pattern.
        #[arg(long)]
        rename_folders: bool,
        
        /// If set, only files will be renamed (default behavior if --rename-folders is not present).
        #[arg(long, conflicts_with = "rename_folders")]
        rename_files: bool, // Added for clarity, conflicts with rename_folders
        
        /// If set, will prevent asking user for confirmation before renaming. Use with caution.
        #[arg(long)]
        no_warnings: bool
    },
}