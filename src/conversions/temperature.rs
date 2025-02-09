use std::str::FromStr;

#[derive(Debug)]
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

impl FromStr for TemperatureUnit {
    type Err = &'static str;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        use TemperatureUnit::*;
        let unit = unit.to_lowercase();

        match unit.as_str() {
            "c" | "celsius" | "centigrade" => Ok(Celsius),
            "f" | "fahrenheit" => Ok(Fahrenheit),
            "k" | "kelvin" => Ok(Kelvin),
            "r" | "rankine" => Ok(Rankine),
            "d" | "delisle" => Ok(Delisle),
            "n" | "newton" => Ok(Newton),
            "re" | "réaumur" | "reaumur" => Ok(Reaumur),
            "ro" | "rømer" | "romer" => Ok(Romer),
            _ => Err("Invalid unit"),
        }
    }
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
