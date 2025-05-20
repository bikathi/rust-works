use clap::Parser;
use cn::input_handler::CliInput;

fn main() {
    let cli_args:CliInput = CliInput::parse();
    
    match std::fs::rename(&cli_args.file_name, "my_new_name.txt") {
        Ok(_) => println!("file renamed"),
        Err(_) => eprintln!("failed to rename file!"),
    };
    dbg!("{}", cli_args);
}
