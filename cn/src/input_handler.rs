use clap::Parser;

#[derive(PartialEq, Debug, Parser, Clone)]
#[command(version, about)]
pub struct CliInput {
    #[arg(short = 'f', long = "file-name")]
    file_name: String,
    
    #[arg(short = 't', long = "to-name")]
    to_name: String,
}

impl CliInput {
    pub fn get_file_name(self: &Self) -> &String {
        &self.file_name
    }
    
    pub fn get_to_name(self: &Self) -> &String {
        &self.to_name
    }
}