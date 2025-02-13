use std::process;
use clap::{Parser, Subcommand, Args};

use conversions::*;

pub mod conversions;

const HELP_TEMPLATE: &str = "
----------------------------------------------------------
{about}
----------------------------------------------------------
({bin} {version})

{usage-heading}{tab}{usage}

{all-args}
";

const SUBCOMMAND_TEMPLATE: &str = "
--------------------------------------
{about}
--------------------------------------

{usage-heading}{tab}{usage}

{all-args}
";

#[derive(Debug, Parser)]
#[command(version, about, long_about=None, help_template=HELP_TEMPLATE)]
pub struct Cmd {
    /// Measurement category
    #[clap(subcommand)]
    pub measurement: Measurement,
}

#[derive(Debug, Subcommand)]
pub enum Measurement {
    /// Convert between distance units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Dist(Fields),
    /// Convert between weight units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Weight(Fields),
    /// Convert between temperature units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Temp(Fields),
    /// Convert between data storage units
    #[command(name="ds", help_template = SUBCOMMAND_TEMPLATE)]
    DataStorage(Fields),
    /// Convert between data transfer units
    #[command(name="dt", help_template = SUBCOMMAND_TEMPLATE)]
    DataTransfer(Fields),
    /// Convert between time units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Time(Fields),
    /// Convert between volume units
    #[command(name="vol", help_template = SUBCOMMAND_TEMPLATE)]
    Volume(Fields),
    /// Convert between area units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Area(Fields),
    /// Convert between frequency units
    #[command(name="freq", help_template = SUBCOMMAND_TEMPLATE)]
    Frequency(Fields),
    /// Convert between force units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Force(Fields),
    /// Convert between energy units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Energy(Fields),
    /// Convert between power units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Power(Fields),
    /// Convert between speed units
    #[command(help_template = SUBCOMMAND_TEMPLATE)]
    Speed(Fields)
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
            Measurement::DataStorage(fields) => handle_conversion(fields, data_storage::convert, data_storage::help_text),
            Measurement::DataTransfer(fields) => handle_conversion(fields, data_transfer::convert, data_transfer::help_text),
            Measurement::Time(fields) => handle_conversion(fields, time::convert, time::help_text),
            Measurement::Volume(fields) => handle_conversion(fields, volume::convert, volume::help_text),
            Measurement::Area(fields) => handle_conversion(fields, area::convert, area::help_text),
            Measurement::Frequency(fields) => handle_conversion(fields, frequency::convert, frequency::help_text),
            Measurement::Force(fields) => handle_conversion(fields, force::convert, force::help_text),
            Measurement::Energy(fields) => handle_conversion(fields, energy::convert, energy::help_text),
            Measurement::Power(fields) => handle_conversion(fields, power::convert, power::help_text),
            Measurement::Speed(fields) => handle_conversion(fields, speed::convert, speed::help_text),
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
