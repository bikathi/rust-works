use rsg::parser::*;
use rsg::crude_generator::CrudeGenerator;
use rsg::urandom_generator::UrandomGenerator;
use rsg::utils::{print_or_store, write_to_file, Generator};
use clap::Parser;

fn main() {
    let randomizer_options: RandomizerOptions = RandomizerOptions::parse();
    let mut string_buffer: Vec<String> = Vec::new();
    
    (0..randomizer_options.total_number).for_each(|_| {
        match &randomizer_options.mode {
            CharactersMode::Uppercase | CharactersMode::Numeric | CharactersMode::Mixed | CharactersMode::Lowercase => {
                print_or_store(
                    CrudeGenerator::generate(&randomizer_options.mode, randomizer_options.length_of_strings), 
                    &mut string_buffer,
                    &randomizer_options.output_file
                );
            },
            CharactersMode::Noisy => {
                print_or_store(
                    UrandomGenerator::generate(&randomizer_options.mode, randomizer_options.length_of_strings), 
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