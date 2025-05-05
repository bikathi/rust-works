use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct RandomizerOptions {
    #[arg(short = 'n', long = "number-of-strings")]
    number_of_strings: usize,
    
    #[arg(short = 'l', long = "length-of-strings")]
    length_of_strings: usize,
    
    #[arg(short = 'm', long = "chars-mode")]
    chars_mode: String, // 1 for numeric only, 2, for uppercase, 3 for lowercase,4 mixed
}

fn main() {
    let randomizer_options: RandomizerOptions = RandomizerOptions::parse();
    dbg!("options: {}", randomizer_options);
}
