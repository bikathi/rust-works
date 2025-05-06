use clap::Parser;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum CharactersMode {
    Numeric,
    Uppercase,
    Lowercase,
    Mixed,
}

impl FromStr for CharactersMode {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" | "numeric"  => Ok(CharactersMode::Numeric),
            "2" | "uppercase" => Ok(CharactersMode::Uppercase),
            "3" | "lowercase" => Ok(CharactersMode::Lowercase),
            "4" | "mixed" => Ok(CharactersMode::Mixed),
            _ => Err("Invaid characters mode. Use 1 (numeric), 2 (uppercase), 3 (lowercase), 4, (mixed)")
        }
    }
}

#[derive(Parser, Debug, PartialEq)]
#[command(version, about)]
pub struct RandomizerOptions {
    #[arg(short = 't', long = "total-random-strings")]
    pub total_number: usize,
    
    #[arg(short = 'l', long = "length-of-strings")]
    pub length_of_strings: usize,
    
    #[arg(short = 'm', long = "chars-mode")]
    pub mode: CharactersMode, // 1 for numeric only, 2 for uppercase, 3 for lowercase, 4 for mixed
}