use std::collections::HashMap;
use std::hash::Hash;

pub trait ConversionUtil {
    type Unit: Eq + Hash + Copy + std::fmt::Debug;
    
    fn generate_conversion_table() -> HashMap<Self::Unit, f64>;
    fn convert(from: Self::Unit, to: Self::Unit, conversion_table: &HashMap<Self::Unit, f64>, value: f64) -> f64;
}