use std::io::Error;

use rsg::parser::*;
use rsg::crude_generator::generate_crude_string;
use rsg::urandom_generator::generate_urandom_string;
use clap::Parser;

fn main() -> Result<(), Error> {
    let randomizer_options: RandomizerOptions = RandomizerOptions::parse();
    
    match &randomizer_options.mode {
        CharactersMode::Uppercase | CharactersMode::Numeric | CharactersMode::Mixed | CharactersMode::Lowercase => {
            (0..randomizer_options.total_number).for_each(|_| println!("{}", generate_crude_string(&randomizer_options)));
        },
        CharactersMode::Noisy => {
            // TODO: The unwrap here potential flaw. Replace it with unwrap_
            (0..randomizer_options.total_number).for_each(|_| println!("{}", generate_urandom_string(&randomizer_options).unwrap()));
        },
    }
    
    Ok(())
    
    // TODO: Allow for writing output to a file
    // TODO: Provide better error handling and communication
}