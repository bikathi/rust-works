use crate::parser::CharactersMode;

pub fn generate_mix(chars_mode: &CharactersMode) -> Vec<char> {
    let numerals: String = String::from("0123456789");
    let lowercase: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let uppercase: String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let symbols: String = String::from("!@#$%^&*()_+");

    match chars_mode {
        CharactersMode::Numeric  => numerals.chars().collect(),
        CharactersMode::Uppercase => uppercase.chars().collect(),
        CharactersMode::Lowercase => lowercase.chars().collect(),
        CharactersMode::Mixed => format!("{}{}{}", numerals, lowercase, uppercase).chars().collect(),
        CharactersMode::Noisy => format!("{}{}{}{}", numerals, lowercase, uppercase, symbols).chars().collect(),
        
    }
}