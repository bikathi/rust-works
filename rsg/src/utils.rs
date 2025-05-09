use crate::parser::CharactersMode;
use std::fs::File;
use std::io::Write;

pub fn print_or_store(random_str: String, buf: &mut Vec<String>, optional_output_file: &Option<String>) {
    match &optional_output_file {
        Some(_) => buf.push(random_str),
        None => println!("{}", random_str),
    }
}

pub fn write_to_file(file: &String, buffer: Vec<String>) {
    // join the buffer into one String with newlines as the separator
    let string_to_write: String = buffer.join("\n");
    
    // create file and get it's handle
    let mut output_file = File::create(file).expect("file creation failed!");
    
    // Write contents to the file
    output_file.write(string_to_write.as_bytes()).expect("write to file failed!");
}

pub trait Generator {
    fn from_mix(chars_mode: &CharactersMode) -> Vec<char> {
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
    
    fn generate(mode: &CharactersMode, length: usize) -> String;
}