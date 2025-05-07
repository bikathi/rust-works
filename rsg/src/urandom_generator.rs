use crate::utils::generate_mix;
use std::{fs::File, io::Read};
use crate::parser::RandomizerOptions;

pub fn generate_urandom_string(options: &RandomizerOptions) -> Result<String, std::io::Error> {
    // generate mix of characters to map the bytes in dev/urandom to
    let mix: Vec<char> = generate_mix(&options.mode);
    
    // get the handle to the dev/urandom
    let mut file_handle: File = File::open("/dev/urandom")?;
    
    // where we store the random String
    let mut random_string = String::new();
    
    // create a buffer with a size equal to the length of each random string
    // then read this exact number of bytes to the buffer
    let mut byte_buffer: Vec<u8> = vec![0u8; options.length_of_strings];
    file_handle.read_exact(&mut byte_buffer)?;
    
    for byte in byte_buffer {
        let index = (byte as usize) % mix.len();
        random_string.push(mix[index])
    }
    
    
    Ok(random_string)
}

