use std::path::PathBuf;

use clap::Parser;
use cn::{
    file_utils::{FileUtils, log_executed_changes},
    input_handler::{CliInput, ModeCommands, get_user_consent},
};
use regex::Regex;

fn main() {
    let cli_args: CliInput = CliInput::parse();

    match cli_args.get_mode_commands() {
        // for single-file renames
        ModeCommands::Single {
            file_name,
            new_name,
        } => {
            FileUtils::rename_file(file_name, new_name);
        }

        // for multi-file renames
        ModeCommands::Bulk {
            directory,
            pattern,
            replacement,
            recursive,
            include_folders,
            no_warnings,
            log_file,
        } => {
            let regex = Regex::new(pattern).unwrap_or_else(|_| panic!("Invalid pattern!"));

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
                *include_folders,
            ) {
                if children.is_empty() {
                    panic!("Found no entries to work on!");
                }

                // count how many items were discovered
                let total_discovered: usize = children.len();

                // get the proposed changes
                let proposed_changes: Vec<(PathBuf, PathBuf)> =
                    FileUtils::derive_new_names(regex, children, replacement);

                // ask for consent if they didn't disable the warning.
                // Highly discourage the user from disabling the warnings
                if *no_warnings {
                    println!(
                        "Consent warning disabled, this is highly discouraged! Proceeding to rename blindly."
                    );
                    FileUtils::execute_rename(&proposed_changes);

                    // log out to file if needed
                    optionally_output_to_logfile(log_file, proposed_changes);
                } else {
                    if let Ok(proceeed_to_apply) = get_user_consent(total_discovered) {
                        if proceeed_to_apply {
                            FileUtils::execute_rename(&proposed_changes);
                            println!("Done!");

                            // log out to file if needed
                            optionally_output_to_logfile(log_file, proposed_changes);
                        } else {
                            println!("Exiting without change!");
                        }

                        return;
                    }
                }
            }
        }

        // for reverting multi-file (bulk) renames
        ModeCommands::Revert {
            log_file,
            no_warnings,
        } => {
            println!("log file: {:?}", log_file);
            println!("no warnings: {no_warnings}");
        }
    }
}

fn optionally_output_to_logfile(log_file: &Option<PathBuf>, change_pair: Vec<(PathBuf, PathBuf)>) {
    if let Some(path_buf) = log_file {
        log_executed_changes(change_pair, path_buf.clone());
        println!("Output written to: {:?}", path_buf);
    }
}
