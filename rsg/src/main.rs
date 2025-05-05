use rsg::CliOptions;
use clap::Parser;

fn main() {
    let randomizer_options: CliOptions = CliOptions::parse();
    dbg!("options: {}", randomizer_options);
}