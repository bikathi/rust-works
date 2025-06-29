use clap::{Parser, Subcommand};
use std::{ffi::OsStr, io::Write, path::PathBuf};

const NAME_FOR_FILE: &'static str = "file";
const NAME_FOR_FOLDER: &'static str = "folder";

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

pub enum DisplayMode {
    PRINT,
    FORMAT,
}

// the available sub-commands for the renaming tool
#[derive(Subcommand, Debug, PartialEq, Clone)]
pub enum ModeCommands {
    /// Use when doing a simple rename to a single file
    Single {
        #[arg(short = 'f', long = "file-name")]
        file_name: PathBuf,

        #[arg(short = 'n', long = "new-name")]
        new_name: String,
    },

    /// Use when renaming multiple files and/or folders in a specified directory based on a regex pattern.
    Bulk {
        /// The base directory in which to perform the bulk renaming.
        #[arg(short, long, value_name = "DIRECTORY_PATH")]
        directory: PathBuf,

        /// The regular expression pattern to match against file/folder names in this base directory.
        #[arg(short, long, value_name = "REGEX_PATTERN")]
        pattern: String,

        /// The replacement string for matched patterns. Use capture groups (e.g., '$1')
        /// to refer to parts of the matched pattern.
        #[arg(long, value_name = "REPLACEMENT_STRING")]
        replacement: String,

        /// If set, the tool will recursively traverse subdirectories.
        #[arg(long)]
        recursive: bool,

        /// If set, folders will also be renamed based on the pattern.
        #[arg(long)]
        include_folders: bool,

        /// If set, will prevent asking user for confirmation before renaming. Use with caution.
        #[arg(long)]
        no_warnings: bool,

        /// If set, will publish all changes to the <LOG_FILE_PATH>
        #[arg(long, value_name = "LOG_FILE_PATH")]
        log_file: Option<PathBuf>,
    },
    
    /// Used to reverse changes performed in bulk mode, based on a log file
    Revert {
        /// The log file that was generated when the bulk rename action was performed
        #[arg(long, value_name = "LOG_FILE_PATH")]
        log_file: Option<PathBuf>,
        
        /// If set, will prevent asking user for confirmation before renaming. Use with caution.
        #[arg(long)]
        no_warnings: bool,
    }
}

pub fn get_user_consent(size_of_changes: usize) -> Result<bool, &'static str> {
    print!("Proceed to apply {} changes? [y/N]: ", size_of_changes);
    std::io::stdout().flush().expect("Failed to flush stdout"); // Ensure the prompt appears before input

    let mut y_on_n = String::new();
    std::io::stdin()
        .read_line(&mut y_on_n)
        .expect("Failed to read line!");

    match y_on_n.trim().to_lowercase().as_str() {
        // Trim to remove newline characters
        "y" => Ok(true),
        "n" => Ok(false),
        _ => Err("Invalid input! Will exit"),
    }
}

pub fn display_proposed_changes(change_pair: (&PathBuf, &PathBuf)) {
    if let (pb, Some(new_os_str)) = (change_pair.0, change_pair.1.file_name()) {
        // the unwrap() calls here is because logic shouldn't fail at this point
        println!(
            "{} [{:?}] [{}] => {new_os_str:?}",
            pb.to_str().unwrap_or_else(|| "?"),
            pb.file_name().unwrap_or_else(|| OsStr::new("?")),
            if pb.is_file() {
                NAME_FOR_FILE
            } else {
                NAME_FOR_FOLDER
            }
        );
    }
}

pub fn print_or_else(change_pair: (&PathBuf, &PathBuf), mode: DisplayMode) -> Option<String> {
    if let (pb, Some(new_os_str)) = (change_pair.0, change_pair.1.file_name()) {
        match mode {
            DisplayMode::PRINT => {
                println!(
                    "{} [{:?}] [{}] => {new_os_str:?}",
                    pb.to_str().unwrap_or_else(|| "?"),
                    pb.file_name().unwrap_or_else(|| OsStr::new("?")),
                    if pb.is_file() {
                        NAME_FOR_FILE
                    } else {
                        NAME_FOR_FOLDER
                    }
                );

                return None;
            }
            DisplayMode::FORMAT => {
                return Some(format!(
                    "{} [{:?}] [{}] => {new_os_str:?}",
                    pb.to_str().unwrap_or_else(|| "?"),
                    pb.file_name().unwrap_or_else(|| OsStr::new("?")),
                    if pb.is_file() {
                        NAME_FOR_FILE
                    } else {
                        NAME_FOR_FOLDER
                    }
                ));
            }
        }
    }

    return None;
}
