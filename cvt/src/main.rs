use clap::Parser;
use cvt::{parser::{ConversionUnit, InputArgs}, time_converter::TimeUnits, utils::ConversionUtil};

fn main() {
    // format of inputs is cvt --unit time --input 10 --from ms to s
    let input_options: InputArgs = InputArgs::parse();
    
    match input_options.unit {
        ConversionUnit::Time => {
            if let Ok((from, to)) = TimeUnits::convert_pair(input_options.from.as_str(), input_options.to.as_str()) {
                dbg!("{} {}", &from, &to);
                let ct = TimeUnits::generate_conversion_table();
                
                println!("{}", TimeUnits::convert(from, to, &ct, input_options.input as f64))
            } else {
                eprintln!("Invalid time units!");
            }
        },
        _ => ()
    }
}
