use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum DistanceUnit {
    Nanometer,
    Micrometer,
    Millimeter,
    Centimeter,
    Decimeter,
    Meter,
    Dekameter,
    Hectometer,
    Kilometer,
    Megameter,
    Gigameter,
    Terameter,
    Thou,
    Inch,
    Foot,
    Yard,
    Mile,
    NauticalMile,
    Fathom,
    Rod,
    Chain,
    Furlong,
    AstronomicalUnit,
    LightYear,
    Parsec,
    Link,
    Cubit,
    Hand,
    Ell,
    Fermi,
    Angstorm,
}

impl FromStr for DistanceUnit {
    type Err = &'static str;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        use DistanceUnit::*;
        let unit = unit.to_lowercase();

        match unit.as_str() {
            "nm" | "nanometer" | "nanometers" | "nanometre" | "nanometres" => Ok(Nanometer),
            "um" | "micrometer" | "micrometers" | "micrometre" | "micrometres" => Ok(Micrometer),
            "mm" | "millimeter" | "millimeters" | "millimetre" | "millimetres" => Ok(Millimeter),
            "cm" | "centimeter" | "centimeters" | "centimetre" | "centimetres" => Ok(Centimeter),
            "dm" | "decimeter" | "decimeters" | "decimetre" | "decimetres" => Ok(Decimeter),
            "m" | "meter" | "meters" | "metre" | "metres" => Ok(Meter),
            "dam" | "dekameter" | "dekameters" | "decameter" | "decameters" => Ok(Dekameter),
            "hm" | "hectometer" | "hectometers" | "hectometre" | "hectometres" => Ok(Hectometer),
            "km" | "kms" | "kilometer" | "kilometers" | "kilometre" | "kilometres" => Ok(Kilometer),
            "Mm" | "megameter" | "megameters" | "megametre" | "megametres" => Ok(Megameter),
            "Gm" | "gigameter" | "gigameters" | "gigametre" | "gigametres" => Ok(Gigameter),
            "Tm" | "terameter" | "terameters" | "terametre" | "terametres" => Ok(Terameter),
            "thou" => Ok(Thou),
            "in" | "inch" | "inches" => Ok(Inch),
            "ft" | "foot" | "feet" => Ok(Foot),
            "yd" | "yard" | "yards" => Ok(Yard),
            "mi" | "mile" | "miles" => Ok(Mile),
            "nmi" | "nautical mile" | "nautical miles" => Ok(NauticalMile),
            "fathom" | "fathoms" => Ok(Fathom),
            "rod" | "rods" => Ok(Rod),
            "chain" | "chains" => Ok(Chain),
            "furlong" | "furlongs" => Ok(Furlong),
            "au" | "astronomical unit" | "astronomical units" => Ok(AstronomicalUnit),
            "ly" | "light year" | "light years" => Ok(LightYear),
            "pc" | "parsec" | "parsecs" => Ok(Parsec),
            "link" | "links" => Ok(Link),
            "cubit" | "cubits" => Ok(Cubit),
            "hand" | "hands" => Ok(Hand),
            "ell" | "ells" => Ok(Ell),
            "fm" | "fermi" => Ok(Fermi),
            "A" | "angstrom" | "angstroms" => Ok(Angstorm),
            _ => Err("Invalid unit"),
        }
    }
}

impl DistanceUnit {
    fn factor(&self) -> f64 {
        use DistanceUnit::*;

        match self {
            Nanometer => 1e-9,
            Micrometer => 1e-6,
            Millimeter => 1e-3,
            Centimeter => 1e-2,
            Decimeter => 1e-1,
            Meter => 1.0,
            Dekameter => 1e1,
            Hectometer => 1e2,
            Kilometer => 1e3,
            Megameter => 1e6,
            Gigameter => 1e9,
            Terameter => 1e12,
            Thou => 0.0000254,
            Inch => 0.0254,
            Foot => 0.3048,
            Yard => 0.9144,
            Mile => 1609.344,
            NauticalMile => 1852.0,
            Fathom => 1.8288,
            Rod => 5.0292,
            Chain => 20.1168,
            Furlong => 201.168,
            Link => 0.201168,
            Cubit => 0.4572,
            Hand => 0.1016,
            Ell => 1.143,
            Fermi => 1e-15,
            Angstorm => 1e-10,
            AstronomicalUnit => 1.495978707e11,
            LightYear => 9.4607e15,
            Parsec => 3.0857e16,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: DistanceUnit = from_unit.parse()?;
    let to: DistanceUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round()/10000.0)
}
