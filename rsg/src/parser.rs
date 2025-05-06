use clap::Parser;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct RandomizerOptions {
    pub total_number: usize,
    pub length_of_strings: usize,
    pub mode: CharactersMode, // 1 for numeric only, 2 for uppercase, 3 for lowercase, 4 for mixed
}

#[cfg(test)]
mod tests {
    
}