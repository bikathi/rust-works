use std::path::PathBuf;

use clap::Parser;

#[derive(PartialEq, Debug, Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct CliInput {
    /// The folder containing the files to re-organize
    #[arg(short = 'b', long = "base-path")]
    base_path: PathBuf,

    /// The criteria to use to organize files: date, name, type, size
    #[arg(short = 'o', long = "organize-by")]
    organize_by: String,

    /// Ordering: Ascending or Descending
    #[arg(short = 'm', long = "mode")]
    mode: String,
}
