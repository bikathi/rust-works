use crate::utils::ConversionUtil;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum DistanceUnits {
    Nanometers,
    Micrometers,
    Millimeters,
    Centimeters,
    Meters,
    Kilometers,
    Inches,
    Feet,
    Yards,
    Miles,
    NauticalMiles,
    LightYears,
    AstronomicalUnits,
    Parsecs
}

impl ConversionUtil for DistanceUnits {
    type Unit = DistanceUnits;
    
    fn match_unit(unit_str: &str) -> Result<Self::Unit, String> {
        match unit_str {
            "nm" | "nanometers" => Ok(DistanceUnits::Nanometers),
            "mic" | "micrometers" => Ok(DistanceUnits::Micrometers),
            "mm" | "millimeters" => Ok(DistanceUnits::Millimeters),
            "cm" | "centimeters" => Ok(DistanceUnits::Centimeters),
            "m" | "meters" => Ok(DistanceUnits::Meters),
            "km" | "kilometers" => Ok(DistanceUnits::Kilometers),
            "in" | "inches" => Ok(DistanceUnits::Inches),
            "ft" | "feet" => Ok(DistanceUnits::Feet),
            "yd" | "yards" => Ok(DistanceUnits::Yards),
            "mi" | "miles" => Ok(DistanceUnits::Miles),
            "nmi" | "nautical miles" => Ok(DistanceUnits::NauticalMiles),
            "ly" | "light years" => Ok(DistanceUnits::LightYears),
            "au" | "astronomical units" => Ok(DistanceUnits::AstronomicalUnits),
            "pc" | "parsecs" => Ok(DistanceUnits::Parsecs),
            _ => Err(format!("Invalid distance unit: {}", unit_str)),
        }
    }
    
    fn generate_conversion_table() -> HashMap<Self::Unit, f64> {
        let mut table = HashMap::new();
        
        // Smallest units
        table.insert(DistanceUnits::Nanometers, 1e-9);   // 1 nm = 0.000000001 meters
        table.insert(DistanceUnits::Micrometers, 1e-6); // 1 µm = 0.000001 meters
        table.insert(DistanceUnits::Millimeters, 0.001); // 1 mm = 0.001 meters
    
        // Standard length units
        table.insert(DistanceUnits::Centimeters, 0.01);    // 1 cm = 0.01 meters
        table.insert(DistanceUnits::Meters, 1.0);         // Base unit
        table.insert(DistanceUnits::Kilometers, 1000.0);  // 1 km = 1000 meters
    
        // Imperial units
        table.insert(DistanceUnits::Inches, 0.0254);      // 1 in = 0.0254 meters
        table.insert(DistanceUnits::Feet, 0.3048);        // 1 ft = 0.3048 meters
        table.insert(DistanceUnits::Yards, 0.9144);       // 1 yd = 0.9144 meters
        table.insert(DistanceUnits::Miles, 1609.344);     // 1 mi = 1609.344 meters
    
        // Nautical units
        table.insert(DistanceUnits::NauticalMiles, 1852.0); // 1 nm = 1852 meters
    
        // Astronomical units
        table.insert(DistanceUnits::LightYears, 9.461e15); // 1 ly = 9.461 × 10^15 meters
        table.insert(DistanceUnits::AstronomicalUnits, 1.496e11); // 1 AU = 1.496 × 10^11 meters
        table.insert(DistanceUnits::Parsecs, 3.086e16);    // 1 parsec = 3.086 × 10^16 meters
    
        table
    }
}