use crate::utils::ConversionUtil;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum WeightUnits {
    Micrograms,
    Milligrams,
    Centigrams,
    Decigrams,
    Grams,
    Decagrams,
    Hectograms,
    Kilograms,
    MetricTons,
    Ounces,
    Pounds,
    Stones,
    USTons,
    UKTons,
    Carats,
}

// implement the conversion_util trait
impl ConversionUtil for WeightUnits {
    type Unit = WeightUnits;
    
    fn match_unit(unit_str: &str) -> Result<Self::Unit, String> {
        match unit_str {
            "mic" | "micrograms" => Ok(WeightUnits::Micrograms),
            "mg" | "milligrams" => Ok(WeightUnits::Milligrams),
            "cg" | "centigrams" => Ok(WeightUnits::Centigrams),
            "dg" | "decigrams" => Ok(WeightUnits::Decigrams),
            "g" | "grams" => Ok(WeightUnits::Grams),
            "dag" | "decagrams" => Ok(WeightUnits::Decagrams),
            "hg" | "hectograms" => Ok(WeightUnits::Hectograms),
            "kg" | "kilograms" => Ok(WeightUnits::Kilograms),
            "t" | "metric tons" => Ok(WeightUnits::MetricTons),
            "oz" | "ounces" => Ok(WeightUnits::Ounces),
            "lb" | "pounds" => Ok(WeightUnits::Pounds),
            "st" | "stones" => Ok(WeightUnits::Stones),
            "ust" | "us tons" => Ok(WeightUnits::USTons),
            "ukt" | "uk tons" => Ok(WeightUnits::UKTons),
            "ct" | "carats" => Ok(WeightUnits::Carats),
            _ => Err(format!("Invalid weight unit: {}", unit_str)),
        }
    }
    
    fn generate_conversion_table() -> HashMap<Self::Unit, f64> {
        let mut table = HashMap::new();
        // Small units
        table.insert(WeightUnits::Micrograms, 1e-6);   // 1 µg = 0.000001 grams
        table.insert(WeightUnits::Milligrams, 0.001);  // 1 mg = 0.001 grams
        table.insert(WeightUnits::Centigrams, 0.01);   // 1 cg = 0.01 grams
        table.insert(WeightUnits::Decigrams, 0.1);     // 1 dg = 0.1 grams
    
        // Base unit
        table.insert(WeightUnits::Grams, 1.0);         // 1 g = 1 gram
        
        // Larger metric units
        table.insert(WeightUnits::Decagrams, 10.0);    // 1 dag = 10 grams
        table.insert(WeightUnits::Hectograms, 100.0);  // 1 hg = 100 grams
        table.insert(WeightUnits::Kilograms, 1000.0);  // 1 kg = 1000 grams
        table.insert(WeightUnits::MetricTons, 1_000_000.0); // 1 metric ton = 1,000,000 grams
    
        // Imperial units
        table.insert(WeightUnits::Ounces, 28.3495);    // 1 oz = 28.3495 grams
        table.insert(WeightUnits::Pounds, 453.592);    // 1 lb = 453.592 grams
        table.insert(WeightUnits::Stones, 6350.293);   // 1 st = 6,350.293 grams
        table.insert(WeightUnits::USTons, 907184.74);  // 1 US ton = 907,184.74 grams
        table.insert(WeightUnits::UKTons, 1_016_046.9088); // 1 UK ton = 1,016,046.9088 grams
    
        // Large-scale weights
        table.insert(WeightUnits::Carats, 0.2);        // 1 ct = 0.2 grams
    
        table
    }
}