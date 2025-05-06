use rsg::parser::*;
use rsg::crude_generator::generate_string;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_options: RandomizerOptions = CliOptions::parse().convert_into()?;
    
    (0..cli_options.total_number).for_each(|_| println!("{}", generate_string(&cli_options)));
    
    Ok(())
    
    // TODO: write up unit tests in the lib folder
    // TODO: create a default mode
    // TODO: Get rid of the RandomizerOptions struct. Use only one struct
    // TODO: Add mode where we can use dev/
}