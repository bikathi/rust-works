use clap::Parser;

#[derive(Debug)]
pub enum CharactersMode {
    Numeric,
    Uppercase,
    Lowercase,
    Mixed,
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct CliOptions {
    #[arg(short = 't', long = "total-random-strings")]
    pub total_strings: usize,
    
    #[arg(short = 'l', long = "length-of-strings")]
    pub length_of_strings: usize,
    
    #[arg(short = 'm', long = "chars-mode")]
    pub chars_mode: i32, // 1 for numeric only, 2 for uppercase, 3 for lowercase, 4 for mixed
}

#[derive(Debug)]
pub struct RandomizerOptions {
    pub total_number: usize,
    pub length_of_strings: usize,
    pub mode: CharactersMode, // 1 for numeric only, 2 for uppercase, 3 for lowercase, 4 for mixed
}