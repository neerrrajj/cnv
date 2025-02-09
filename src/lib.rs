use clap::{Parser, Subcommand, Args};

use conversions::distance;
use conversions::temperature;

pub mod conversions;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
pub struct Cmd {
    /// Measurement category. Supported types: 'dist' (distance)
    #[clap(subcommand)]
    pub measurement: Measurement,
}

#[derive(Debug, Subcommand)]
pub enum Measurement {
    /// Convert between distance units
    Dist(Fields),
    /// Convert between weight units
    Weight(Fields),
    /// Convert between temperature units
    Temp(Fields)
}

#[derive(Debug, Args)]
pub struct Fields {
    /// The numerical value to convert
    pub value: f64,
    /// The unit to convert from
    pub from_unit: String,
    /// The unit to convert to
    pub to_unit: String,
}

impl Cmd {
    pub fn execute(&self) -> Result<(f64, &str, f64, &str), &'static str> {
        match &self.measurement {
            Measurement::Dist(fields) => {
                let result = distance::convert(fields.value, &fields.from_unit, &fields.to_unit)?;
                Ok((fields.value, fields.from_unit.as_str(), result, fields.to_unit.as_str()))
            },
            Measurement::Weight(_) => Err("not yet implemented"),
            Measurement::Temp(fields) => {
                let result = temperature::convert(fields.value, &fields.from_unit, &fields.to_unit)?;
                Ok((fields.value, fields.from_unit.as_str(), result, fields.to_unit.as_str()))
            },
        }
    }
}
