use std::{env::Args, str::FromStr};

use conversions::distance;

pub mod conversions;

#[derive(Debug)]
pub struct Cmd {
    pub measurement: Measurement,
    pub value: f64,
    pub from_unit: String,
    pub to_unit: String,
}

#[derive(Debug)]
pub enum Measurement {
    Dist,
    Weight
}

impl FromStr for Measurement {
    type Err = &'static str;

    fn from_str(measurement: &str) -> Result<Self, Self::Err> {
        let text = measurement.to_lowercase();
        match text.as_str() {
            "dist" => Ok(Measurement::Dist),
            "weight" => Ok(Measurement::Weight),
            _ => Err("Invalid measurement type")
        }
    }
}

impl Cmd {
    pub fn new(mut args: Args) -> Result<Cmd, &'static str> {
        if args.len() != 5 {
            return Err("Usage: cnv <category> <value> <from_unit> <to_unit>");
        }
        args.next();
        let measurement = match args.next() {
            Some(v) => v.parse()?,
            None => return Err("Unexpected error occured -_-"),
        };
        let value = match args.next() {
            Some(v) => v.parse().map_err(|_| "Invalid number")?,
            None => return Err("Unexpected error occured -_-"),
        };
        let from_unit = match args.next() {
            Some(v) => v,
            None => return Err("Unexpected error occured -_-"),
        };
        let to_unit = match args.next() {
            Some(v) => v,
            None => return Err("Unexpected error occured -_-"),
        };
        
        Ok(Cmd {
            measurement,
            value,
            from_unit,
            to_unit,
        })
    }

    pub fn execute(&self) -> Result<f64, &'static str> {
        match self.measurement {
            Measurement::Dist => distance::convert(self.value, &self.from_unit, &self.to_unit),
            Measurement::Weight => Err("not yet implemented")
        }
    }
}
