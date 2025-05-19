use crate::utils::ConversionUtil;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TimeUnits {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

impl TimeUnits {
    fn matcher(unit_str: &str) -> Result<TimeUnits, String> {
        match unit_str {
            "ms" | "milliseconds" => Ok(TimeUnits::Milliseconds),
            "s" | "seconds" => Ok(TimeUnits::Seconds),
            "m" | "minutes" => Ok(TimeUnits::Minutes),
            "h" | "hours" => Ok(TimeUnits::Hours),
            "d" | "days" => Ok(TimeUnits::Days),
            _ => Err(format!("Invalid time unit: {}", unit_str)),
        }
    }
    
    pub fn convert_pair(input_unit: &str, output_unit: &str) -> Result<(TimeUnits, TimeUnits), String> {
        let input_unit = Self::matcher(input_unit)?;
        let output_unit = Self::matcher(output_unit)?;
        
        Ok((input_unit, output_unit))
    } 
}

// implement the conversion_util trait
impl ConversionUtil for TimeUnits {
    type Unit = TimeUnits;
    
    fn generate_conversion_table() -> HashMap<Self::Unit, f64> {
        let mut ct = HashMap::new();
        ct.insert(Self::Milliseconds, 0.0_f64);
        ct
    }
    
    fn convert(from: Self::Unit, to: Self::Unit, conversion_table: &HashMap<Self::Unit, f64>, value: f64) -> f64 {
        10.2_f64
    }
}