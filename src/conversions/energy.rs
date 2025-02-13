use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum EnergyUnit {
    Joule,
    Kilojoule,
    Megajoule,
    Gigajoule,
    Terajoule,
    Petajoule,
    Exajoule,
    Zettajoule,
    Yottajoule,
    Electronvolt,
    Kiloelectronvolt,
    Megaelectronvolt,
    Gigaelectronvolt,
    Teraelectronvolt,
    Calorie,
    Kilocalorie,
    Megacalorie,
    Gigacalorie,
    WattHour,
    KilowattHour,
    MegawattHour,
    GigawattHour,
    TerawattHour,
    BritishThermalUnit,
    Therm,
    Erg,
    FootPound,
    InchPound,
    NewtonMeter,
    WattSecond,
    HorsepowerHour,
}

pub struct UnitDef {
    variant: EnergyUnit,
    name: &'static str,
    aliases: &'static [&'static str],
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef { variant: EnergyUnit::Joule, name: "Joule", aliases: &["J", "joule", "joules"] },
    UnitDef { variant: EnergyUnit::Kilojoule, name: "Kilojoule", aliases: &["kJ", "kilojoule", "kilojoules"] },
    UnitDef { variant: EnergyUnit::Megajoule, name: "Megajoule", aliases: &["MJ", "megajoule", "megajoules"] },
    UnitDef { variant: EnergyUnit::Gigajoule, name: "Gigajoule", aliases: &["GJ", "gigajoule", "gigajoules"] },
    UnitDef { variant: EnergyUnit::Terajoule, name: "Terajoule", aliases: &["TJ", "terajoule", "terajoules"] },
    UnitDef { variant: EnergyUnit::Petajoule, name: "Petajoule", aliases: &["PJ", "petajoule", "petajoules"] },
    UnitDef { variant: EnergyUnit::Exajoule, name: "Exajoule", aliases: &["EJ", "exajoule", "exajoules"] },
    UnitDef { variant: EnergyUnit::Zettajoule, name: "Zettajoule", aliases: &["ZJ", "zettajoule", "zettajoules"] },
    UnitDef { variant: EnergyUnit::Yottajoule, name: "Yottajoule", aliases: &["YJ", "yottajoule", "yottajoules"] },
    UnitDef { variant: EnergyUnit::Electronvolt, name: "Electronvolt", aliases: &["eV", "electronvolt", "electronvolts"] },
    UnitDef { variant: EnergyUnit::Kiloelectronvolt, name: "Kiloelectronvolt", aliases: &["keV", "kiloelectronvolt", "kiloelectronvolts"] },
    UnitDef { variant: EnergyUnit::Megaelectronvolt, name: "Megaelectronvolt", aliases: &["MeV", "megaelectronvolt", "megaelectronvolts"] },
    UnitDef { variant: EnergyUnit::Gigaelectronvolt, name: "Gigaelectronvolt", aliases: &["GeV", "gigaelectronvolt", "gigaelectronvolts"] },
    UnitDef { variant: EnergyUnit::Teraelectronvolt, name: "Teraelectronvolt", aliases: &["TeV", "teraelectronvolt", "teraelectronvolts"] },
    UnitDef { variant: EnergyUnit::Calorie, name: "Calorie", aliases: &["cal", "calorie", "calories"] },
    UnitDef { variant: EnergyUnit::Kilocalorie, name: "Kilocalorie", aliases: &["kcal", "kilocalorie", "kilocalories"] },
    UnitDef { variant: EnergyUnit::Megacalorie, name: "Megacalorie", aliases: &["Mcal", "megacalorie", "megacalories"] },
    UnitDef { variant: EnergyUnit::Gigacalorie, name: "Gigacalorie", aliases: &["Gcal", "gigacalorie", "gigacalories"] },
    UnitDef { variant: EnergyUnit::WattHour, name: "Watt-Hour", aliases: &["Wh", "watt hour", "watt-hour", "watt hours", "watt-hours"] },
    UnitDef { variant: EnergyUnit::KilowattHour, name: "Kilowatt-Hour", aliases: &["kWh", "kilowatt hour", "kilowatt-hour", "kilowatt hours", "kilowatt-hours"] },
    UnitDef { variant: EnergyUnit::MegawattHour, name: "Megawatt-Hour", aliases: &["MWh", "megawatt hour", "megawatt-hour", "megawatt hours", "megawatt-hours"] },
    UnitDef { variant: EnergyUnit::GigawattHour, name: "Gigawatt-Hour", aliases: &["GWh", "gigawatt hour", "gigawatt-hour", "gigawatt hours", "gigawatt-hours"] },
    UnitDef { variant: EnergyUnit::TerawattHour, name: "Terawatt Hour", aliases: &["TWh", "terawatt hour", "terawatt hours", "terawatt-hour", "terawatt-hours"] },
    UnitDef { variant: EnergyUnit::BritishThermalUnit, name: "British Thermal Unit", aliases: &["BTU", "btu", "british thermal unit", "british thermal units"] },
    UnitDef { variant: EnergyUnit::Therm, name: "Therm", aliases: &["therm", "therms"] },
    UnitDef { variant: EnergyUnit::FootPound, name: "Foot-Pound", aliases: &["ft-lb", "foot pound", "foot pounds", "foot-pound", "foot-pounds"] },
    UnitDef { variant: EnergyUnit::InchPound, name: "Inch Pound", aliases: &["in-lb", "inch pound", "inch pounds", "inch-pound", "inch-pounds"] },
    UnitDef { variant: EnergyUnit::Erg, name: "Erg", aliases: &["erg", "ergs"] },
    UnitDef { variant: EnergyUnit::NewtonMeter, name: "Newton Meter", aliases: &["Nm", "newton meter", "newton meters", "newton-meter", "newton-meters"] },
    UnitDef { variant: EnergyUnit::WattSecond, name: "Watt Second", aliases: &["Ws", "watt second", "watt seconds", "watt-second", "watt-seconds"] },
    UnitDef { variant: EnergyUnit::HorsepowerHour, name: "Horsepower Hour", aliases: &["hp-h", "horsepower hour", "horsepower hours", "horsepower-hour", "horsepower-hours"] },
];

impl_conversion_traits!(EnergyUnit, UNIT_DEFS);

pub fn help_text() -> String {
    EnergyUnit::generate_help_text()
}

impl EnergyUnit {
    fn factor(&self) -> f64 {
        use EnergyUnit::*;

        match self {
            Joule => 1.0,
            Kilojoule => 1e3,
            Megajoule => 1e6,
            Gigajoule => 1e9,
            Terajoule => 1e12,
            Petajoule => 1e15,
            Exajoule => 1e18,
            Zettajoule => 1e21,
            Yottajoule => 1e24,
            Electronvolt => 1.60218e-19,
            Kiloelectronvolt => 1.60218e-16,
            Megaelectronvolt => 1.60218e-13,
            Gigaelectronvolt => 1.60218e-10,
            Teraelectronvolt => 1.60218e-7,
            Calorie => 4.184,
            Kilocalorie => 4184.0,
            Megacalorie => 4.184e6,
            Gigacalorie => 4.184e9,
            WattHour => 3600.0,
            KilowattHour => 3.6e6,
            MegawattHour => 3.6e9,
            GigawattHour => 3.6e12,
            TerawattHour => 3.6e15,
            BritishThermalUnit => 1055.06,
            Therm => 1.05506e8,
            FootPound => 1.35582,
            InchPound => 0.112985,
            Erg => 1e-7,
            NewtonMeter => 1.0,
            WattSecond => 1.0,
            HorsepowerHour => 2.68452e6,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: EnergyUnit = from_unit.parse()?;
    let to: EnergyUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round() / 10000.0)
}
