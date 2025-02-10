use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
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

pub struct UnitDef {
    variant: DistanceUnit,
    name: &'static str,
    aliases: &'static[&'static str]
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef {
        variant: DistanceUnit::Nanometer,
        name: "Nanometer",
        aliases: &["nm", "nanometer", "nanometers", "nanometre", "nanometres"],
    },
    UnitDef {
        variant: DistanceUnit::Micrometer,
        name: "Micrometer",
        aliases: &["um", "micrometer", "micrometers", "micrometre", "micrometres"],
    },
    UnitDef {
        variant: DistanceUnit::Millimeter,
        name: "Millimeter",
        aliases: &["mm", "millimeter", "millimeters", "millimetre", "millimetres"],
    },
    UnitDef {
        variant: DistanceUnit::Centimeter,
        name: "Centimeter",
        aliases: &["cm", "centimeter", "centimeters", "centimetre", "centimetres"],
    },
    UnitDef {
        variant: DistanceUnit::Decimeter,
        name: "Decimeter",
        aliases: &["dm", "decimeter", "decimeters", "decimetre", "decimetres"],
    },
    UnitDef {
        variant: DistanceUnit::Meter,
        name: "Meter",
        aliases: &["m", "meter", "meters", "metre", "metres"],
    },
    UnitDef {
        variant: DistanceUnit::Dekameter,
        name: "Dekameter",
        aliases: &["dam", "dekameter", "dekameters", "decameter", "decameters"],
    },
    UnitDef {
        variant: DistanceUnit::Hectometer,
        name: "Hectometer",
        aliases: &["hm", "hectometer", "hectometers", "hectometre", "hectometres"],
    },
    UnitDef {
        variant: DistanceUnit::Kilometer,
        name: "Kilometer",
        aliases: &["km", "kms", "kilometer", "kilometers", "kilometre", "kilometres"],
    },
    UnitDef {
        variant: DistanceUnit::Megameter,
        name: "Megameter",
        aliases: &["Mm", "megameter", "megameters", "megametre", "megametres"],
    },
    UnitDef {
        variant: DistanceUnit::Gigameter,
        name: "Gigameter",
        aliases: &["Gm", "gigameter", "gigameters", "gigametre", "gigametres"],
    },
    UnitDef {
        variant: DistanceUnit::Terameter,
        name: "Terameter",
        aliases: &["Tm", "terameter", "terameters", "terametre", "terametres"],
    },
    UnitDef {
        variant: DistanceUnit::Thou,
        name: "Thou",
        aliases: &["thou"],
    },
    UnitDef {
        variant: DistanceUnit::Inch,
        name: "Inch",
        aliases: &["in", "inch", "inches"],
    },
    UnitDef {
        variant: DistanceUnit::Foot,
        name: "Foot",
        aliases: &["ft", "foot", "feet"],
    },
    UnitDef {
        variant: DistanceUnit::Yard,
        name: "Yard",
        aliases: &["yd", "yard", "yards"],
    },
    UnitDef {
        variant: DistanceUnit::Mile,
        name: "Mile",
        aliases: &["mi", "mile", "miles"],
    },
    UnitDef {
        variant: DistanceUnit::NauticalMile,
        name: "Nautical Mile",
        aliases: &["nmi", "nautical mile", "nautical miles"],
    },
    UnitDef {
        variant: DistanceUnit::Fathom,
        name: "Fathom",
        aliases: &["fathom", "fathoms"],
    },
    UnitDef {
        variant: DistanceUnit::Rod,
        name: "Rod",
        aliases: &["rod", "rods"],
    },
    UnitDef {
        variant: DistanceUnit::Chain,
        name: "Chain",
        aliases: &["chain", "chains"],
    },
    UnitDef {
        variant: DistanceUnit::Furlong,
        name: "Furlong",
        aliases: &["furlong", "furlongs"],
    },
    UnitDef {
        variant: DistanceUnit::AstronomicalUnit,
        name: "Astronomical Unit",
        aliases: &["au", "astronomical unit", "astronomical units"],
    },
    UnitDef {
        variant: DistanceUnit::LightYear,
        name: "Light Year",
        aliases: &["ly", "light year", "light years"],
    },
    UnitDef {
        variant: DistanceUnit::Parsec,
        name: "Parsec",
        aliases: &["pc", "parsec", "parsecs"],
    },
    UnitDef {
        variant: DistanceUnit::Link,
        name: "Link",
        aliases: &["link", "links"],
    },
    UnitDef {
        variant: DistanceUnit::Cubit,
        name: "Cubit",
        aliases: &["cubit", "cubits"],
    },
    UnitDef {
        variant: DistanceUnit::Hand,
        name: "Hand",
        aliases: &["hand", "hands"],
    },
    UnitDef {
        variant: DistanceUnit::Ell,
        name: "Ell",
        aliases: &["ell", "ells"],
    },
    UnitDef {
        variant: DistanceUnit::Fermi,
        name: "Fermi",
        aliases: &["fm", "fermi"],
    },
    UnitDef {
        variant: DistanceUnit::Angstorm,
        name: "Angstorm",
        aliases: &["A", "angstorm", "angstorms"],
    },
];

impl_conversion_traits!(DistanceUnit, UNIT_DEFS);

pub fn help_text() -> String {
    DistanceUnit::generate_help_text()
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
