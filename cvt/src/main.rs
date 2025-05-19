use clap::Parser;
use cvt::{
    parser::{ConversionUnit, InputArgs}, 
    time_converter::TimeUnits, 
    utils::ConversionUtil,  
    distance_converter::DistanceUnits
};

fn main() {
    // format of inputs is cvt --unit time --input 10 --from ms to s
    let input_options: InputArgs = InputArgs::parse();
    
    // disallow 0's to avoid possible division by zero errors
    if input_options.input <= 0 {
        eprintln!("Invalid input");
        return;
    }
    
    match input_options.unit {
        ConversionUnit::Time => {
            conversion_dispatcher::<TimeUnits>(input_options);
        },
        ConversionUnit::Distance => {
            conversion_dispatcher::<DistanceUnits>(input_options);
        },
    }
}

fn conversion_dispatcher<A>(input_options: InputArgs) 
where 
    A: ConversionUtil + PartialEq {
    if let Ok((from, to)) = A::convert_pair(input_options.from.as_str(), input_options.to.as_str()) {
        let ct = A::generate_conversion_table();
        match A::convert(from, to, &ct, input_options.input as f64) {
            Ok(res) => println!("{}", res),
            Err(message) => eprintln!("{}", message),
        }
    } else {
        eprintln!("Invalid measurement units!");
    }
}
