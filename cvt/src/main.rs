use clap::Parser;
use cvt::parser::InputArgs;

fn main() {
    // format of inputs is cvt --unit Time --input 10
    let input_options: InputArgs = InputArgs::parse();
    dbg!("Input units: {}", input_options);
}
