use rsg::{ CliOptions, RandomizerOptions };
use clap::Parser;

fn main() {
    let cli_options: RandomizerOptions = match CliOptions::parse().convert_into() {
        Ok(cli_options) => { cli_options },
        _ => panic!("Failed to parse provided options!"),
    };
    
    dbg!("options: {}", cli_options);
}