use std::io::Error;

use rsg::parser::*;
use rsg::crude_generator::generate_crude_string;
use rsg::urandom_generator::generate_urandom_string;
use rsg::utils::{print_or_store, write_to_file};
use clap::Parser;

fn main() {
    let randomizer_options: RandomizerOptions = RandomizerOptions::parse();
    let mut string_buffer: Vec<String> = Vec::new();
    
    (0..randomizer_options.total_number).for_each(|_| {
        match &randomizer_options.mode {
            CharactersMode::Uppercase | CharactersMode::Numeric | CharactersMode::Mixed | CharactersMode::Lowercase => {
                print_or_store(
                    generate_crude_string(&randomizer_options), 
                    &mut string_buffer,
                    &randomizer_options.output_file
                );
            },
            CharactersMode::Noisy => {
                print_or_store(
                    generate_urandom_string(&randomizer_options).unwrap(), 
                    &mut string_buffer,
                    &randomizer_options.output_file
                );
            },
        }
        
    });
    
    if let Some(file) = &randomizer_options.output_file {
        write_to_file(file, string_buffer)
    }
    
    // TODO: Provide better error handling and communication
}