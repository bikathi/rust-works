use crate::parser::CharactersMode;

pub fn generate_mix(chars_mode: &CharactersMode) -> String {
    let numerals: String = String::from("0123456789");
    let lowercase: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let uppercase: String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let symbols: String = String::from("!@#$%^&*()_+");

    match chars_mode {
        CharactersMode::Numeric  => numerals,
        CharactersMode::Uppercase => uppercase,
        CharactersMode::Lowercase => lowercase,
        CharactersMode::Mixed => format!("{}{}{}", numerals, lowercase, uppercase),
        CharactersMode::Noisy => format!("{}{}{}{}", numerals, lowercase, uppercase, symbols),
        
    }
}