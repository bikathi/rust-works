use crate::parser::{CharactersMode, RandomizerOptions};
use rand::Rng;

pub fn generate_mix(chars_mode: &CharactersMode) -> String {
    let numerals: String = String::from("0123456789");
    let lowercase: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let uppercase: String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

    match chars_mode {
        CharactersMode::Numeric  => numerals,
        CharactersMode::Uppercase => uppercase,
        CharactersMode::Lowercase => lowercase,
        CharactersMode::Mixed => format!("{}{}{}", numerals, lowercase, uppercase),
    }
}

pub fn generate_string(options: &RandomizerOptions) -> String {
    let scope = generate_mix(&options.mode);
    let mut result = String::new();
    
    for _ in 0..options.length_of_strings {
        let secret_number = rand::rng().random_range(0..scope.len());
        result.push_str(scope.get(secret_number..=secret_number).unwrap());
    }
    result
}