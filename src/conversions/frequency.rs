use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum FrequencyUnit {
    Yoctohertz,
    Zeptohertz,
    Attohertz,
    Femtohertz,
    Picohertz,
    Nanohertz,
    Microhertz,
    Millihertz,
    Centihertz,
    Decihertz,
    Hertz,
    Decahertz,
    Hectohertz,
    Kilohertz,
    Megahertz,
    Gigahertz,
    Terahertz,
    Petahertz,
    Exahertz,
    Zettahertz,
    Yottahertz,
    RevolutionsPerMinute,
    RevolutionsPerHour,
    CyclesPerMinute,
    CyclesPerHour,
    CyclesPerDay,
    BeatsPerMinute,
    Savart,
}

pub struct UnitDef {
    variant: FrequencyUnit,
    name: &'static str,
    aliases: &'static [&'static str],
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef { variant: FrequencyUnit::Yoctohertz, name: "Yoctohertz", aliases: &["yHz", "yoctohertz", "yoctoHz"] },
    UnitDef { variant: FrequencyUnit::Zeptohertz, name: "Zeptohertz", aliases: &["zHz", "zeptohertz", "zeptoHz"] },
    UnitDef { variant: FrequencyUnit::Attohertz, name: "Attohertz", aliases: &["aHz", "attohertz", "attoHz"] },
    UnitDef { variant: FrequencyUnit::Femtohertz, name: "Femtohertz", aliases: &["fHz", "femtohertz", "femtoHz"] },
    UnitDef { variant: FrequencyUnit::Picohertz, name: "Picohertz", aliases: &["pHz", "picohertz", "picoHz"] },
    UnitDef { variant: FrequencyUnit::Nanohertz, name: "Nanohertz", aliases: &["nHz", "nanohertz", "nanoHz"] },
    UnitDef { variant: FrequencyUnit::Microhertz, name: "Microhertz", aliases: &["µHz", "uHz", "microhertz", "microHz"] },
    UnitDef { variant: FrequencyUnit::Millihertz, name: "Millihertz", aliases: &["mHz", "millihertz", "milliHz"] },
    UnitDef { variant: FrequencyUnit::Centihertz, name: "Centihertz", aliases: &["cHz", "centihertz", "centiHz"] },
    UnitDef { variant: FrequencyUnit::Decihertz, name: "Decihertz", aliases: &["dHz", "decihertz", "deciHz"] },
    UnitDef { variant: FrequencyUnit::Hertz, name: "Hertz", aliases: &["Hz", "hz", "hertz", "cycles per second", "cycles per sec", "c/s", "cps", "cycles/s", "rps", "RPS", "rev/s", "revs/s", "revs per sec", "revolutions/s", "revolutions per sec", "revolutions per second", "revs per second", "r/s"] },
    UnitDef { variant: FrequencyUnit::Decahertz, name: "Decahertz", aliases: &["daHz", "decahertz", "decaHz"] },
    UnitDef { variant: FrequencyUnit::Hectohertz, name: "Hectohertz", aliases: &["hHz", "hectohertz", "hectoHz"] },
    UnitDef { variant: FrequencyUnit::Kilohertz, name: "Kilohertz", aliases: &["kHz", "kilohertz", "kilocycle", "kc", "kc/s"] },
    UnitDef { variant: FrequencyUnit::Megahertz, name: "Megahertz", aliases: &["MHz", "megahertz", "megacycle", "Mc", "Mc/s"] },
    UnitDef { variant: FrequencyUnit::Gigahertz, name: "Gigahertz", aliases: &["GHz", "gigahertz", "gigacycle", "Gc", "Gc/s"] },
    UnitDef { variant: FrequencyUnit::Terahertz, name: "Terahertz", aliases: &["THz", "terahertz", "teracycle", "Tc", "Tc/s", "fresnel", "Fresnel"] },
    UnitDef { variant: FrequencyUnit::Petahertz, name: "Petahertz", aliases: &["PHz", "petahz", "peta", "petahertz", "petahertzs"] },
    UnitDef { variant: FrequencyUnit::Exahertz, name: "Exahertz", aliases: &["EHz", "exa", "exahz", "exahertz", "exahertzs"] },
    UnitDef { variant: FrequencyUnit::Zettahertz, name: "Zettahertz", aliases: &["ZHz", "zetahz", "zetta", "zettahertz", "zettahertzs"] },
    UnitDef { variant: FrequencyUnit::Yottahertz, name: "Yottahertz", aliases: &["YHz", "yottahz", "yotta", "yottahertz", "yottahertzs"] },
    UnitDef { variant: FrequencyUnit::RevolutionsPerMinute, name: "RevolutionsPerMinute", aliases: &["rpm", "RPM", "rev/min", "revs/min", "revs per min", "revs per minute", "revolutions/min", "revolutions per min", "revolutions per minute"] },
    UnitDef { variant: FrequencyUnit::RevolutionsPerHour, name: "RevolutionsPerHour", aliases: &["rph", "RPH", "rev/h", "revs/h", "revs per hour", "revolutions/h", "revolutions per h", "revolutions per hour"] },
    UnitDef { variant: FrequencyUnit::CyclesPerMinute, name: "CyclesPerMinute", aliases: &["CPM", "cpm", "cycles per minute", "cycles per min", "cycles/min"] },
    UnitDef { variant: FrequencyUnit::CyclesPerHour, name: "CyclesPerHour", aliases: &["CPH", "cph", "cycles per hour", "cycles/h"] },
    UnitDef { variant: FrequencyUnit::CyclesPerDay, name: "CyclesPerDay", aliases: &["CPD", "cpd", "cycles per day", "cycles/day"] },
    UnitDef { variant: FrequencyUnit::BeatsPerMinute, name: "BeatsPerMinute", aliases: &["BPM", "bpm", "beats per minute", "beats per min", "beats/min"] },
    UnitDef { variant: FrequencyUnit::Savart, name: "Savart", aliases: &["savart", "Savarts", "savarts"] },
];

impl_conversion_traits!(FrequencyUnit, UNIT_DEFS);

pub fn help_text() -> String {
    FrequencyUnit::generate_help_text()
}

impl FrequencyUnit {
    fn factor(&self) -> f64 {
        use FrequencyUnit::*;
        
        match self {
            Yoctohertz => 1e-24,
            Zeptohertz => 1e-21,
            Attohertz => 1e-18,
            Femtohertz => 1e-15,
            Picohertz => 1e-12,
            Nanohertz => 1e-9,
            Microhertz => 1e-6,
            Millihertz => 1e-3,
            Centihertz => 1e-2,
            Decihertz => 1e-1,
            Hertz => 1.0,
            Decahertz => 1e1,
            Hectohertz => 1e2,
            Kilohertz => 1e3,
            Megahertz => 1e6,
            Gigahertz => 1e9,
            Terahertz => 1e12,
            Petahertz => 1e15,
            Exahertz => 1e18,
            Zettahertz => 1e21,
            Yottahertz => 1e24,
            RevolutionsPerMinute => 1.0 / 60.0,
            RevolutionsPerHour => 1.0 / 3600.0,
            CyclesPerMinute => 1.0 / 60.0,
            CyclesPerHour => 1.0 / 3600.0,
            CyclesPerDay => 1.0 / 86400.0,
            BeatsPerMinute => 1.0 / 60.0,
            Savart => 1.0 / 300.0,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: FrequencyUnit = from_unit.parse()?;
    let to: FrequencyUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round() / 10000.0)
}
