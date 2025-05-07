use clap::Parser;
use std::{fmt::Display, str::FromStr};

#[derive(Parser, Debug, PartialEq)]
#[command(version, about)]
pub struct RandomizerOptions {
    #[arg(short = 't', long = "total-random-strings")]
    pub total_number: usize,
    
    #[arg(short = 'l', long = "length-of-strings")]
    pub length_of_strings: usize,
    
    #[arg(short = 'm', long = "chars-mode", default_value_t = CharactersMode::Mixed)]
    pub mode: CharactersMode, // 1 for numeric only, 2 for uppercase, 3 for lowercase, 4 for mixed
    
    #[arg(short = 'o', long = "output-file", default_value = None)]
    pub output_file: Option<String>
}

#[derive(Debug, PartialEq, Clone)]
pub enum CharactersMode {
    Numeric,
    Uppercase,
    Lowercase,
    Mixed,
    Noisy,
}

impl FromStr for CharactersMode {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" | "numeric"  => Ok(CharactersMode::Numeric),
            "2" | "uppercase" => Ok(CharactersMode::Uppercase),
            "3" | "lowercase" => Ok(CharactersMode::Lowercase),
            "4" | "mixed" => Ok(CharactersMode::Mixed),
            "5" | "noisy" => Ok(CharactersMode::Noisy),
            _ => Err("Invaid characters mode. Use 1 (numeric), 2 (uppercase), 3 (lowercase), 4, (mixed)")
        }
    }
}

impl Display for CharactersMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharactersMode::Numeric => write!(f, "numeric"),
            CharactersMode::Uppercase => write!(f, "uppercase"),
            CharactersMode::Lowercase => write!(f, "lowercase"),
            CharactersMode::Mixed => write!(f, "mixed"),
            CharactersMode::Noisy => write!(f, "noisy"),
        }
    }
}