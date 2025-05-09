use crate::utils::Generator;
use std::{fs::File, io::Read};
use crate::parser::CharactersMode;

pub struct UrandomGenerator;

impl Generator for UrandomGenerator {
    fn generate(mode: &CharactersMode, length: usize) -> String {
        // generate mix of characters to map the bytes in dev/urandom to
        let mix: Vec<char> = Self::from_mix(mode);
        
        // get the handle to the dev/urandom
        let mut file_handle: File = File::open("/dev/urandom").expect("Failed to access urandom!");
        
        // where we store the random String
        let mut random_string = String::new();
        
        // create a buffer with a size equal to the length of each random string
        // then read this exact number of bytes to the buffer
        let mut byte_buffer: Vec<u8> = vec![0u8; length];
        file_handle.read_exact(&mut byte_buffer).expect("Failed to read sized byte!");
        
        for byte in byte_buffer {
            let index = (byte as usize) % mix.len();
            random_string.push(mix[index])
        }
        
        random_string
    }
}

