use std::{str::FromStr, fmt::Display};
use clap::Parser;

#[derive(PartialEq, Debug, Parser, Clone)]
#[command(version, about)]
pub struct InputArgs {
    #[arg(short = 'u', long = "unit")]
    pub unit: ConversionUnit,
    
    #[arg(short = 'i', long = "input")]
    pub input: usize,
    
    #[arg(short = 'f', long = "from")]
    pub from: String,
    
    #[arg(short = 't', long = "to")]
    pub to: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ConversionUnit {
    Time,
    Distance,
    Weight
}

impl FromStr for ConversionUnit {
    type Err = &'static str;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "time" => Ok(ConversionUnit::Time),
            "distance" => Ok(ConversionUnit::Distance),
            "weight" => Ok(ConversionUnit::Weight),
            _ => Err("Invalid input format!"),
        }
    }
}

impl Display for ConversionUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionUnit::Time => write!(f, "Time"),
            ConversionUnit::Distance => write!(f, "Distance"),
            ConversionUnit::Weight => write!(f, "Weight"),
        }
    }
}