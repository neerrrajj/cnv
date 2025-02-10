use std::process;
use clap::{Parser, Subcommand, Args};

use conversions::distance;
use conversions::weight;
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
    #[arg(required_unless_present("list"))]
    pub value: Option<f64>,
    /// The unit to convert from
    #[arg(required_unless_present("list"))]
    pub from_unit: Option<String>,
    /// The unit to convert to
    #[arg(required_unless_present("list"))]
    pub to_unit: Option<String>,
    /// List all units of this measurement type
    #[arg(long, short='L', global=true)]
    pub list: bool
}

impl Cmd {
    pub fn execute(&self) -> Result<(f64, &str, f64, &str), &'static str> {
        match &self.measurement {
            Measurement::Dist(fields) => handle_conversion(fields, distance::convert, distance::help_text),
            Measurement::Weight(fields) => handle_conversion(fields, weight::convert, weight::help_text),
            Measurement::Temp(fields) => handle_conversion(fields, temperature::convert, temperature::help_text),
        }
    }
}

fn handle_conversion(
    fields: &Fields, 
    convert: fn(f64, &str, &str) -> Result<f64, &'static str>,
    help_text: fn() -> String
) -> Result<(f64, &str, f64, &str), &'static str> {

    if fields.list {
        println!("{}", help_text());
        process::exit(0);
    }

    let value = fields.value.ok_or("Value required when not listing units")?;
    let from = fields.from_unit.as_deref().ok_or("From unit required")?;
    let to = fields.to_unit.as_deref().ok_or("To unit required")?;

    let result = convert(value, from, to)?;
    Ok((value, from, result, to))
}
