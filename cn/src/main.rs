use clap::Parser;
use cn::{
    input_handler::CliInput,
    file_utils::FileUtils,
};

fn main() {
    let cli_args:CliInput = CliInput::parse();
    
    FileUtils::rename_file(&cli_args.get_file_name(), &cli_args.get_to_name().as_str());
}
