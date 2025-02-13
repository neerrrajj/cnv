use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum PowerUnit {
    Watt,
    Deciwatt,
    Centiwatt,
    Milliwatt,
    Microwatt,
    Nanowatt,
    Picowatt,
    Femtowatt,
    Attowatt,
    Zeptowatt,
    Yoctowatt,
    Kilowatt,
    Decawatt,
    Hectowatt,
    Megawatt,
    Gigawatt,
    Terawatt,
    Petawatt,
    Exawatt,
    Zettawatt,
    Yottawatt,
    ErgPerSecond,
    Poncelet,
    ChevalVapeur,
    CaloriePerHour,
    Horsepower,
    MetricHorsepower,
    ElectricHorsepower,
    BoilerHorsepower,
    FootPoundForcePerMinute,
    BTUPerHour,
    MBTUPerHour,
    TonOfRefrigeration,
    UKTonOfRefrigeration,
    Lusec,
    Clusec,
    Donkeypower, 
}

pub struct UnitDef {
    variant: PowerUnit,
    name: &'static str,
    aliases: &'static [&'static str],
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef { variant: PowerUnit::Yoctowatt, name: "Yoctowatt", aliases: &["yW", "yoctowatt", "yoctowatts"] },
    UnitDef { variant: PowerUnit::Zeptowatt, name: "Zeptowatt", aliases: &["zW", "zeptowatt", "zeptowatts"] },
    UnitDef { variant: PowerUnit::Attowatt, name: "Attowatt", aliases: &["aW", "attowatt", "attowatts"] },
    UnitDef { variant: PowerUnit::Femtowatt, name: "Femtowatt", aliases: &["fW", "femtowatt", "femtowatts"] },
    UnitDef { variant: PowerUnit::Picowatt, name: "Picowatt", aliases: &["pW", "picowatt", "picowatts"] },
    UnitDef { variant: PowerUnit::Nanowatt, name: "Nanowatt", aliases: &["nW", "nanowatt", "nanowatts"] },
    UnitDef { variant: PowerUnit::Microwatt, name: "Microwatt", aliases: &["uW", "microwatt", "microwatts"] },
    UnitDef { variant: PowerUnit::Milliwatt, name: "Milliwatt", aliases: &["mW", "milliwatt", "milliwatts"] },
    UnitDef { variant: PowerUnit::Centiwatt, name: "Centiwatt", aliases: &["cW", "centiwatt", "centiwatts"] },
    UnitDef { variant: PowerUnit::Deciwatt, name: "Deciwatt", aliases: &["dW", "deciwatt", "deciwatts"] },
    UnitDef { variant: PowerUnit::Watt, name: "Watt", aliases: &["W", "watt", "watts"] },
    UnitDef { variant: PowerUnit::Decawatt, name: "Decawatt", aliases: &["daW", "decawatt", "decawatts"] },
    UnitDef { variant: PowerUnit::Hectowatt, name: "Hectowatt", aliases: &["hW", "hectowatt", "hectowatts"] },
    UnitDef { variant: PowerUnit::Kilowatt, name: "Kilowatt", aliases: &["kW", "kilowatt", "kilowatts"] },
    UnitDef { variant: PowerUnit::Megawatt, name: "Megawatt", aliases: &["mW", "megawatt", "megawatts"] },
    UnitDef { variant: PowerUnit::Gigawatt, name: "Gigawatt", aliases: &["gW", "gigawatt", "gigawatts"] },
    UnitDef { variant: PowerUnit::Terawatt, name: "Terawatt", aliases: &["tW", "terawatt", "terawatts"] },
    UnitDef { variant: PowerUnit::Petawatt, name: "Petawatt", aliases: &["pW", "petawatt", "petawatts"] },
    UnitDef { variant: PowerUnit::Exawatt, name: "Exawatt", aliases: &["eW", "exawatt", "exawatts"] },
    UnitDef { variant: PowerUnit::Zettawatt, name: "Zettawatt", aliases: &["zW", "zettawatt", "zettawatts"] },
    UnitDef { variant: PowerUnit::Yottawatt, name: "Yottawatt", aliases: &["yW", "yottawatt", "yottawatts"] },
    UnitDef { variant: PowerUnit::ErgPerSecond, name: "ErgPerSecond", aliases: &["erg/s", "erg per second", "ergs per second"] },
    UnitDef { variant: PowerUnit::Poncelet, name: "Poncelet", aliases: &["p", "poncelet", "poncelets"] },
    UnitDef { variant: PowerUnit::ChevalVapeur, name: "ChevalVapeur", aliases: &["CV", "ch", "cheval vapeur", "cheval-vapeur"] },
    UnitDef { variant: PowerUnit::CaloriePerHour, name: "CaloriePerHour", aliases: &["cal/h", "calorie per hour", "calories per hour"] },
    UnitDef { variant: PowerUnit::Horsepower, name: "Horsepower", aliases: &["hp", "HP", "horsepower", "horsepowers", "mechanical horsepower"] },
    UnitDef { variant: PowerUnit::MetricHorsepower, name: "MetricHorsepower", aliases: &["PS", "ps", "metric hp", "pferdestarke", "cv", "metric horsepower"] },
    UnitDef { variant: PowerUnit::ElectricHorsepower, name: "ElectricHorsepower", aliases: &["EHP", "electric hp", "electric horsepower"] },
    UnitDef { variant: PowerUnit::BoilerHorsepower, name: "BoilerHorsepower", aliases: &["BHP", "bhp", "boiler hp", "boiler horsepower"] },
    UnitDef { variant: PowerUnit::FootPoundForcePerMinute, name: "FootPoundForcePerMinute", aliases: &["ft·lbf/min", "foot-pound per minute", "foot-pounds per minute"] },
    UnitDef { variant: PowerUnit::BTUPerHour, name: "BTUPerHour", aliases: &["BTU/h", "Btu/h", "btu per hour", "BTUH"] },
    UnitDef { variant: PowerUnit::MBTUPerHour, name: "MBTUPerHour", aliases: &["MBTU/h", "mbtu/h", "thousand btu per hour", "MBH"] },
    UnitDef { variant: PowerUnit::TonOfRefrigeration, name: "TonOfRefrigeration", aliases: &["TR", "ton of refrigeration", "tons of refrigeration", "refrigeration ton"] },
    UnitDef { variant: PowerUnit::UKTonOfRefrigeration, name: "UKTonOfRefrigeration", aliases: &["UK TR", "imperial ton of refrigeration"] },
    UnitDef { variant: PowerUnit::Lusec, name: "Lusec", aliases: &["lusec", "lusecs", "L·µmHg/s"] },
    UnitDef { variant: PowerUnit::Clusec, name: "Clusec", aliases: &["clusec", "clusecs", "centilusec"] },
    UnitDef { variant: PowerUnit::Donkeypower, name: "Donkeypower", aliases: &["donkeypower", "donkeypowers", "dp"] },
];

impl_conversion_traits!(PowerUnit, UNIT_DEFS);

pub fn help_text() -> String {
    PowerUnit::generate_help_text()
}

impl PowerUnit {
    fn factor(&self) -> f64 {
        use PowerUnit::*;

        match self {
            Yoctowatt => 1e-24,
            Zeptowatt => 1e-21,
            Attowatt => 1e-18,
            Femtowatt => 1e-15,
            Picowatt => 1e-12,
            Nanowatt => 1e-9,
            Microwatt => 1e-6,
            Milliwatt => 1e-3,
            Centiwatt => 1e-2,
            Deciwatt => 1e-1,
            Watt => 1.0,
            Decawatt => 1e1,
            Hectowatt => 1e2,
            Kilowatt => 1e3,
            Megawatt => 1e6,
            Gigawatt => 1e9,
            Terawatt => 1e12,
            Petawatt => 1e15,
            Exawatt => 1e18,
            Zettawatt => 1e21,
            Yottawatt => 1e24,
            ErgPerSecond => 1e-7,
            Poncelet => 980.665,
            ChevalVapeur => 735.49875,
            CaloriePerHour => 1.1622222222222e-3,
            Horsepower => 745.69987158,
            MetricHorsepower => 735.49875,
            ElectricHorsepower => 746.0,
            BoilerHorsepower => 9809.5,
            FootPoundForcePerMinute => 0.0225969658,
            BTUPerHour => 0.29307107017222,
            MBTUPerHour => 293.07107017222,
            TonOfRefrigeration => 3516.85284,
            UKTonOfRefrigeration => 3934.880789024,
            Lusec => 0.00133322,
            Clusec => 0.0000133322,
            Donkeypower => 250.0,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: PowerUnit = from_unit.parse()?;
    let to: PowerUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round() / 10000.0)
}
