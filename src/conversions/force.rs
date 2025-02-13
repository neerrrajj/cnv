use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum ForceUnit {
    Yoctonewton,
    Zeptonewton,
    Attonewton,
    Femtonewton,
    Piconewton,
    Nanonewton,
    Micronewton,
    Millinewton,
    Centinewton,
    Decinewton,
    Newton,
    Decanewton,
    Hectonewton,
    Kilonewton,
    Meganewton,
    Giganewton,
    Teranewton,
    Dyne,
    KilogramForce,
    GramForce,
    TonneForce,
    Sthene,
    PoundForce,
    Poundal,
    OunceForce,
    Kip,
    ShortTonForce,
    LongTonForce,
}

pub struct UnitDef {
    variant: ForceUnit,
    name: &'static str,
    aliases: &'static [&'static str],
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef { variant: ForceUnit::Yoctonewton, name: "Yoctonewton", aliases: &["yN", "yoctonewton", "yoctonewtons"] },
    UnitDef { variant: ForceUnit::Zeptonewton, name: "Zeptonewton", aliases: &["zN", "zeptonewton", "zeptonewtons"] },
    UnitDef { variant: ForceUnit::Attonewton, name: "Attonewton", aliases: &["aN", "attonewton", "attonewtons"] },
    UnitDef { variant: ForceUnit::Femtonewton, name: "Femtonewton", aliases: &["fN", "femtonewton", "femtonewtons"] },
    UnitDef { variant: ForceUnit::Piconewton, name: "Piconewton", aliases: &["pN", "piconewton", "piconewtons"] },
    UnitDef { variant: ForceUnit::Nanonewton, name: "Nanonewton", aliases: &["nN", "nanonewton", "nanonewtons"] },
    UnitDef { variant: ForceUnit::Micronewton, name: "Micronewton", aliases: &["µN", "uN", "micronewton", "micronewtons"] },
    UnitDef { variant: ForceUnit::Millinewton, name: "Millinewton", aliases: &["mN", "millinewton", "millinewtons"] },
    UnitDef { variant: ForceUnit::Centinewton, name: "Centinewton", aliases: &["cN", "centinewton", "centinewtons"] },
    UnitDef { variant: ForceUnit::Decinewton, name: "Decinewton", aliases: &["dN", "decinewton", "decinewtons"] },
    UnitDef { variant: ForceUnit::Newton, name: "Newton", aliases: &["N", "newton", "newtons"] },
    UnitDef { variant: ForceUnit::Decanewton, name: "Decanewton", aliases: &["daN", "decanewton", "decanewtons"] },
    UnitDef { variant: ForceUnit::Hectonewton, name: "Hectonewton", aliases: &["hN", "hectonewton", "hectonewtons"] },
    UnitDef { variant: ForceUnit::Kilonewton, name: "Kilonewton", aliases: &["kN", "kilonewton", "kilonewtons"] },
    UnitDef { variant: ForceUnit::Meganewton, name: "Meganewton", aliases: &["MN", "meganewton", "meganewtons"] },
    UnitDef { variant: ForceUnit::Giganewton, name: "Giganewton", aliases: &["GN", "giganewton", "giganewtons"] },
    UnitDef { variant: ForceUnit::Teranewton, name: "Teranewton", aliases: &["TN", "teranewton", "teranewtons"] },
    UnitDef { variant: ForceUnit::Dyne, name: "Dyne", aliases: &["dyn", "dyne", "dynes"] },
    UnitDef { variant: ForceUnit::KilogramForce, name: "KilogramForce", aliases: &["kgf", "kilogram-force", "kilograms-force", "kilopond", "kp", "kg-f", "grave-force", "Gf"] },
    UnitDef { variant: ForceUnit::GramForce, name: "GramForce", aliases: &["gf", "gram-force", "grams-force", "pond", "p", "milligrave-force", "mGf"] },
    UnitDef { variant: ForceUnit::TonneForce, name: "TonneForce", aliases: &["tf", "tonne-force", "tonnes-force", "metric ton-force"] },
    UnitDef { variant: ForceUnit::Sthene, name: "Sthene", aliases: &["sn", "sthene", "sthenes"] },
    UnitDef { variant: ForceUnit::PoundForce, name: "PoundForce", aliases: &["lbf", "pound-force", "pounds-force", "lb-f", "poundforce"] },
    UnitDef { variant: ForceUnit::Poundal, name: "Poundal", aliases: &["pdl", "poundal", "poundals"] },
    UnitDef { variant: ForceUnit::OunceForce, name: "OunceForce", aliases: &["ozf", "ounce-force", "ounces-force", "oz-f"] },
    UnitDef { variant: ForceUnit::Kip, name: "Kip", aliases: &["kip", "kips", "kilopound-force"] },
    UnitDef { variant: ForceUnit::ShortTonForce, name: "ShortTonForce", aliases: &["short ton-force", "us ton-force", "short tons-force"] },
    UnitDef { variant: ForceUnit::LongTonForce, name: "LongTonForce", aliases: &["long ton-force", "uk ton-force", "long tons-force"] },
];

impl_conversion_traits!(ForceUnit, UNIT_DEFS);

pub fn help_text() -> String {
    ForceUnit::generate_help_text()
}

impl ForceUnit {
    fn factor(&self) -> f64 {
        use ForceUnit::*;
        
        match self {
            Yoctonewton => 1e-24,
            Zeptonewton => 1e-21,
            Attonewton => 1e-18,
            Femtonewton => 1e-15,
            Piconewton => 1e-12,
            Nanonewton => 1e-9,
            Micronewton => 1e-6,
            Millinewton => 1e-3,
            Centinewton => 1e-2,
            Decinewton => 1e-1,
            Newton => 1.0,
            Decanewton => 1e1,
            Hectonewton => 1e2,
            Kilonewton => 1e3,
            Meganewton => 1e6,
            Giganewton => 1e9,
            Teranewton => 1e12,
            Dyne => 1e-5,
            KilogramForce => 9.80665,
            GramForce => 0.00980665,
            TonneForce => 9806.65,
            Sthene => 1e3,
            PoundForce => 4.4482216152605,
            Poundal => 0.138254954376,
            OunceForce => 0.278013850953,
            Kip => 4448.2216152605,
            ShortTonForce => 8896.443230521,
            LongTonForce => 9964.01641818352,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: ForceUnit = from_unit.parse()?;
    let to: ForceUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round() / 10000.0)
}
