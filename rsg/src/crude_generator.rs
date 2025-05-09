use crate::parser::CharactersMode;
use rand::Rng;
use crate::utils::Generator;

pub struct CrudeGenerator;

impl Generator for CrudeGenerator {
    fn generate(mode: &CharactersMode, length: usize) -> String {
        let scope: Vec<char> = Self::from_mix(mode);
        let mut result = String::new();
        
        for _ in 0..length {
            let secret_number = rand::rng().random_range(0..scope.len());
            result.push(scope[secret_number]);
        }
        result
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    
    fn mix(mode: &CharactersMode) -> Vec<char> {
        generate_mix(mode)
    }
    
    fn check_mix_mode(mode: CharactersMode, mix: &Vec<char>) -> bool {
        match mode {
            CharactersMode::Lowercase => {
                for c in mix {
                    if c.is_lowercase() {
                        return true
                    }
                }
                
                false
            },
            
            CharactersMode::Uppercase => {
                for c in mix {
                    if c.is_uppercase() {
                        return true
                    }
                }
                
                false
            },
            
            CharactersMode::Numeric => {
                for c in mix {
                    if c.is_numeric() {
                        return true
                    }
                }
                
                false
            },
            
            CharactersMode::Mixed => {
                let mut has_lowercase = false;
                let mut has_uppercase = false;
                let mut has_numeric = false;
                
                for c in mix {
                    if c.is_lowercase() {
                        has_lowercase =  true;
                        continue;
                    }
                    
                    if c.is_uppercase() {
                        has_uppercase = true;
                        continue;
                    }
                    
                    if c.is_numeric() {
                        has_numeric = true;
                        continue;
                    }
                }
                
                return has_numeric && has_uppercase && has_lowercase
            }
            
            CharactersMode::Noisy => {
                false
            }
        }
    }
    
    #[test]
    fn should_return_non_empty_mix() {
        let mix: Vec<char> = mix(&CharactersMode::Mixed);
        assert!(!mix.is_empty());
    }
    
    #[test]
    fn should_generate_reliable_mix() {
        // check mix only contains lowercase for a test case
        let mode: CharactersMode = CharactersMode::Lowercase;
        let mix: Vec<char> = mix(&mode);
        assert_eq!(check_mix_mode(mode, &mix), true);
    }
    
    #[test]
    fn should_generate_random_uppercase_string() {
        let random_str = generate_crude_string(
            &RandomizerOptions { 
                total_number: 1, 
                length_of_strings: 10, 
                mode: CharactersMode::Uppercase 
            }
        );
        
        random_str.chars().for_each(|s| assert!(s.is_uppercase()));
        println!("{}", random_str);
    }
    
    #[test]
    fn should_generate_random_lowercase_string() {
        let random_str = generate_crude_string(
            &RandomizerOptions { 
                total_number: 1, 
                length_of_strings: 10, 
                mode: CharactersMode::Lowercase 
            }
        );
        
        random_str.chars().for_each(|s| assert!(s.is_lowercase()));
    }
    
    #[test]
    fn should_generate_random_numeric() {
        let random_str = generate_crude_string(
            &RandomizerOptions { 
                total_number: 1, 
                length_of_strings: 10, 
                mode: CharactersMode::Numeric 
            }
        );
        
        random_str.chars().for_each(|s| assert!(s.is_numeric()));
    }
    
    #[test]
    fn should_generate_sized_random_string() {
        let random_str = generate_crude_string(
            &RandomizerOptions { 
                total_number: 1, 
                length_of_strings: 10, 
                mode: CharactersMode::Uppercase 
            }
        );
        
        assert!(random_str.len() == 10);
    }
}
*/