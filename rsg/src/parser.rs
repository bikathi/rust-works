use clap::Parser;

#[derive(Debug, PartialEq)]
pub enum CharactersMode {
    Numeric,
    Uppercase,
    Lowercase,
    Mixed,
}

impl CharactersMode {
    fn delegate_mode(char_mode: &i32) -> Result<Self, &'static str> {
        match char_mode {
            1 => Ok(Self::Numeric),
            2 => Ok(Self::Uppercase),
            3 => Ok(Self::Lowercase),
            4 => Ok(Self::Mixed),
            _ => Err("Failed to parse character mode!"),
        }
    }
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

impl CliOptions {
    pub fn convert_into(self: &Self) -> Result<RandomizerOptions, &str> {
        let char_mode = CharactersMode::delegate_mode(&self.chars_mode)?;
        Ok(RandomizerOptions {
            total_number: self.total_strings,
            length_of_strings: self.length_of_strings,
            mode: char_mode,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct RandomizerOptions {
    pub total_number: usize,
    pub length_of_strings: usize,
    pub mode: CharactersMode, // 1 for numeric only, 2 for uppercase, 3 for lowercase, 4 for mixed
}

#[cfg(test)]
mod test {
    use crate::parser::{CharactersMode, RandomizerOptions};

    use super::CliOptions;

    #[test]
    #[should_panic]
    fn should_panic_on_conversion() {
        match CliOptions::convert_into(&CliOptions { total_strings: 1, length_of_strings: 1, chars_mode: 10 }) {
            Ok(_) => (),
            Err(_) => panic!("converter panicked!"),
        }
    }
    
    #[test]
    fn should_correctly_convert() {
        let mut converted_matches = false;
        
        if let Ok(randomizer_opts) = CliOptions::convert_into(&CliOptions { total_strings: 1, length_of_strings: 1, chars_mode: 4 }) {
            if randomizer_opts == (RandomizerOptions {
                total_number: 1, 
                length_of_strings: 1,
                mode: CharactersMode::Mixed,
                
            }) {
                converted_matches = true
            }
        }
        
        assert_eq!(converted_matches, true);
    }
}