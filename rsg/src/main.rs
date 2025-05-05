use rsg::*;
use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_options: RandomizerOptions = CliOptions::parse().convert_into()?;
    
    for _ in 0..cli_options.total_number {
        println!("{}", generate_string(&cli_options));
    };
    
    Ok(())
}