use std::collections::HashMap;
use std::hash::Hash;

pub trait ConversionUtil {
    type Unit: Eq + Hash + Copy + std::fmt::Debug;
    
    fn match_unit(unit_str: &str) -> Result<Self::Unit, String>;
    fn generate_conversion_table() -> HashMap<Self::Unit, f64>;
    
    // default functions
    fn convert_pair(input_unit: &str, output_unit: &str) -> Result<(Self::Unit, Self::Unit), String> {
        let input_unit = Self::match_unit(input_unit)?;
        let output_unit = Self::match_unit(output_unit)?;
        
        Ok((input_unit, output_unit))
    }
    
    fn convert(from: Self::Unit, to: Self::Unit, conversion_table: &HashMap<Self::Unit, f64>, value: f64) -> Result<f64, String> {
        // Get the conversion factor for the 'from' unit
        let from_factor = conversion_table.get(&from);
        // Get the conversion factor for the 'to' unit
        let to_factor = conversion_table.get(&to);

        match (from_factor, to_factor) {
            (Some(from_f), Some(to_f)) => {
                // Convert `from` unit to base unit (seconds) first, then convert to `to` unit
                Ok((value * from_f) / to_f)
            }
            _ => Err(String::from("Cannot divide by zero!")), // Return None if either 'from' or 'to' unit is not in the table
        }
    }
}