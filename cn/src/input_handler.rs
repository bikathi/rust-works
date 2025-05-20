use clap::Parser;

#[derive(PartialEq, Debug, Parser, Clone)]
#[command(version, about)]
pub struct CliInput {
    #[arg(short = 'f', long = "file-name")]
    pub file_name: String,
}