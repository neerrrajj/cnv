use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Delisle,
    Newton,
    Reaumur,
    Romer,
}

pub struct UnitDef {
    variant: TemperatureUnit,
    name: &'static str,
    aliases: &'static[&'static str]
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef {
        variant: TemperatureUnit::Celsius,
        name: "Celsius",
        aliases: &["c", "celsius", "centigrade"],
    },
    UnitDef {
        variant: TemperatureUnit::Fahrenheit,
        name: "Fahrenheit",
        aliases: &["f", "fahrenheit"],
    },
    UnitDef {
        variant: TemperatureUnit::Kelvin,
        name: "Kelvin",
        aliases: &["k", "kelvin"],
    },
    UnitDef {
        variant: TemperatureUnit::Rankine,
        name: "Rankine",
        aliases: &["r", "rankine"],
    },
    UnitDef {
        variant: TemperatureUnit::Delisle,
        name: "Delisle",
        aliases: &["d", "delisle"],
    },
    UnitDef {
        variant: TemperatureUnit::Newton,
        name: "Newton",
        aliases: &["n", "newton"],
    },
    UnitDef {
        variant: TemperatureUnit::Reaumur,
        name: "Réaumur",
        aliases: &["re", "réaumur", "reaumur"],
    },
    UnitDef {
        variant: TemperatureUnit::Romer,
        name: "Rømer",
        aliases: &["ro", "rømer", "romer"],
    },
];

impl_conversion_traits!(TemperatureUnit, UNIT_DEFS);

pub fn help_text() -> String {
    TemperatureUnit::generate_help_text()
}

impl TemperatureUnit {
    fn to_celsius(&self, value: f64) -> f64 {
        use TemperatureUnit::*;
        match self {
            Celsius => value,
            Fahrenheit => (value - 32.0) * 5.0 / 9.0,
            Kelvin => value - 273.15,
            Rankine => (value - 491.67) * 5.0 / 9.0,
            Delisle => 100.0 - (value * 2.0 / 3.0),
            Newton => value * 100.0 / 33.0,
            Reaumur => value * 5.0 / 4.0,
            Romer => (value - 7.5) * 40.0 / 21.0,
        }
    }

    fn from_celsius(&self, value: f64) -> f64 {
        use TemperatureUnit::*;
        match self {
            Celsius => value,
            Fahrenheit => value * 9.0 / 5.0 + 32.0,
            Kelvin => value + 273.15,
            Rankine => (value + 273.15) * 9.0 / 5.0,
            Delisle => (100.0 - value) * 3.0 / 2.0,
            Newton => value * 33.0 / 100.0,
            Reaumur => value * 4.0 / 5.0,
            Romer => value * 21.0 / 40.0 + 7.5,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: TemperatureUnit = from_unit.parse()?;
    let to: TemperatureUnit = to_unit.parse()?;

    let celsius_value = from.to_celsius(value);
    let result = to.from_celsius(celsius_value);

    Ok((result * 10000.0).round() / 10000.0)
}
