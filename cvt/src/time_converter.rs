use crate::utils::ConversionUtil;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TimeUnits {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Nanoseconds,
    Microseconds,
    Weeks,
    Months,
    Centuries,
    Years,
    Decades,
    Millennia,
}

// implement the conversion_util trait
impl ConversionUtil for TimeUnits {
    type Unit = TimeUnits;
    
    fn match_unit(unit_str: &str) -> Result<Self::Unit, String> {
        match unit_str {
            "ns" | "nanoseconds" => Ok(TimeUnits::Nanoseconds),
            "mic" | "microseconds" => Ok(TimeUnits::Microseconds),
            "ms" | "milliseconds" => Ok(TimeUnits::Milliseconds),
            "s" | "seconds" => Ok(TimeUnits::Seconds),
            "m" | "minutes" => Ok(TimeUnits::Minutes),
            "h" | "hours" => Ok(TimeUnits::Hours),
            "d" | "days" => Ok(TimeUnits::Days),
            "w" | "weeks" => Ok(TimeUnits::Weeks),
            "mo" | "months" => Ok(TimeUnits::Months),
            "y" | "years" => Ok(TimeUnits::Years),
            "dec" | "decades" => Ok(TimeUnits::Decades),
            "c" | "centuries" => Ok(TimeUnits::Centuries),
            "ka" | "millennia" => Ok(TimeUnits::Millennia), // "ka" is commonly used for thousands of years
            _ => Err(format!("Invalid time unit: {}", unit_str)),
        }
    }
    
    fn generate_conversion_table() -> HashMap<Self::Unit, f64> {
        let mut table = HashMap::new();
        // Sub-second units
        table.insert(TimeUnits::Nanoseconds, 1e-9);   // 1 ns = 0.000000001 seconds
        table.insert(TimeUnits::Microseconds, 1e-6); // 1 µs = 0.000001 seconds
        table.insert(TimeUnits::Milliseconds, 0.001); // 1 ms = 0.001 seconds
    
        // Standard time units
        table.insert(TimeUnits::Seconds, 1.0);        // Base unit
        table.insert(TimeUnits::Minutes, 60.0);       // 1 min = 60 seconds
        table.insert(TimeUnits::Hours, 3600.0);       // 1 hour = 3600 seconds
        table.insert(TimeUnits::Days, 86400.0);       // 1 day = 86400 seconds
        table.insert(TimeUnits::Weeks, 604800.0);     // 1 week = 604800 seconds
        table.insert(TimeUnits::Months, 2_629_746.0); // 1 month ≈ 2,629,746 seconds (average across all months)
        table.insert(TimeUnits::Years, 31_556_952.0); // 1 year = 31,556,952 seconds (accounting for leap years)
    
        // Longer durations
        table.insert(TimeUnits::Decades, 315_569_520.0); // 1 decade = 315,569,520 seconds
        table.insert(TimeUnits::Centuries, 3_155_695_200.0); // 1 century = 3,155,695,200 seconds
        table.insert(TimeUnits::Millennia, 31_556_952_000.0); // 1 millennium = 31,556,952,000 seconds
    
        table
    }
}