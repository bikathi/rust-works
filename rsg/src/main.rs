use rsg::parser::*;
use rsg::crude_generator::generate_string;
use clap::Parser;

fn main() {
    let cli_options: RandomizerOptions = RandomizerOptions::parse();
    
    (0..cli_options.total_number).for_each(|_| println!("{}", generate_string(&cli_options)));
    
    // TODO: Add mode where we can use dev/
}