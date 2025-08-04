use clap::Parser;

use crate::input::CliInput;

mod input;

fn main() {
    let cli_args = CliInput::parse();
    println!("provided args: {cli_args:?}");
}
