use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum WeightUnit {
    Microgram,
    Milligram,
    Gram,
    Kilogram,
    MetricTon,
    Megagram,
    Gigagram,
    Teragram,
    Petagram,
    Exagram,
    Zettagram,
    Yottagram,
    Ounce,
    Pound,
    Stone,
    USShortTon,
    ImperialTon,
    Carat,
    Grain,
    Dram,
    Cwt,
    Pennyweight,
    TroyOunce,
    Slug,
    Obol,
    Scruple,
    Tola,
    Baht,
    Momme,
}

pub struct UnitDef {
    variant: WeightUnit,
    name: &'static str,
    aliases: &'static[&'static str]
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef {
        variant: WeightUnit::Microgram,
        name: "Microgram",
        aliases: &["mcg", "microgram", "micrograms"],
    },
    UnitDef {
        variant: WeightUnit::Milligram,
        name: "Milligram",
        aliases: &["mg", "milligram", "milligrams"],
    },
    UnitDef {
        variant: WeightUnit::Gram,
        name: "Gram",
        aliases: &["g", "gram", "grams"],
    },
    UnitDef {
        variant: WeightUnit::Kilogram,
        name: "Kilogram",
        aliases: &["kg", "kgs", "kilogram", "kilograms"],
    },
    UnitDef {
        variant: WeightUnit::MetricTon,
        name: "Metric Ton",
        aliases: &["t", "tonne", "metricton", "metrictons"],
    },
    UnitDef {
        variant: WeightUnit::Megagram,
        name: "Megagram",
        aliases: &["Mg", "megagram", "megagrams"],
    },
    UnitDef {
        variant: WeightUnit::Gigagram,
        name: "Gigagram",
        aliases: &["Gg", "gigagram", "gigagrams"],
    },
    UnitDef {
        variant: WeightUnit::Teragram,
        name: "Teragram",
        aliases: &["Tg", "teragram", "teragrams"],
    },
    UnitDef {
        variant: WeightUnit::Petagram,
        name: "Petagram",
        aliases: &["Pg", "petagram", "petagrams"],
    },
    UnitDef {
        variant: WeightUnit::Exagram,
        name: "Exagram",
        aliases: &["Eg", "exagram", "exagrams"],
    },
    UnitDef {
        variant: WeightUnit::Zettagram,
        name: "Zettagram",
        aliases: &["Zg", "zettagram", "zettagrams"],
    },
    UnitDef {
        variant: WeightUnit::Yottagram,
        name: "Yottagram",
        aliases: &["Yg", "yottagram", "yottagrams"],
    },
    UnitDef {
        variant: WeightUnit::Ounce,
        name: "Ounce",
        aliases: &["oz", "ounce", "ounces"],
    },
    UnitDef {
        variant: WeightUnit::Pound,
        name: "Pound",
        aliases: &["lb", "lbs", "pound", "pounds"],
    },
    UnitDef {
        variant: WeightUnit::Stone,
        name: "Stone",
        aliases: &["st", "stone", "stones"],
    },
    UnitDef {
        variant: WeightUnit::USShortTon,
        name: "US Short Ton",
        aliases: &["us ton", "us short ton", "short ton"],
    },
    UnitDef {
        variant: WeightUnit::ImperialTon,
        name: "Imperial Ton",
        aliases: &["uk ton", "imperial ton", "long ton"],
    },
    UnitDef {
        variant: WeightUnit::Carat,
        name: "Carat",
        aliases: &["ct", "carat", "carats"],
    },
    UnitDef {
        variant: WeightUnit::Grain,
        name: "Grain",
        aliases: &["gr", "grain", "grains"],
    },
    UnitDef {
        variant: WeightUnit::Dram,
        name: "Dram",
        aliases: &["dr", "dram", "drams"],
    },
    UnitDef {
        variant: WeightUnit::Cwt,
        name: "Hundredweight",
        aliases: &["cwt", "hundred weight"],
    },
    UnitDef {
        variant: WeightUnit::Pennyweight,
        name: "Pennyweight",
        aliases: &["dwt", "penny weight", "penny weights"],
    },
    UnitDef {
        variant: WeightUnit::TroyOunce,
        name: "Troy Ounce",
        aliases: &["ozt", "troy ounce", "troy ounces"],
    },
    UnitDef {
        variant: WeightUnit::Slug,
        name: "Slug",
        aliases: &["slug"],
    },
    UnitDef {
        variant: WeightUnit::Obol,
        name: "Obol",
        aliases: &["obol"],
    },
    UnitDef {
        variant: WeightUnit::Scruple,
        name: "Scruple",
        aliases: &["scruple"],
    },
    UnitDef {
        variant: WeightUnit::Tola,
        name: "Tola",
        aliases: &["tola"],
    },
    UnitDef {
        variant: WeightUnit::Baht,
        name: "Baht",
        aliases: &["baht"],
    },
    UnitDef {
        variant: WeightUnit::Momme,
        name: "Momme",
        aliases: &["momme"],
    },
];

impl_conversion_traits!(WeightUnit, UNIT_DEFS);

pub fn help_text() -> String {
    WeightUnit::generate_help_text()
}

impl WeightUnit {
    fn factor(&self) -> f64 {
        use WeightUnit::*;

        match self {
            Microgram => 1e-6,
            Milligram => 1e-3,
            Gram => 1.0,
            Kilogram => 1e3,
            MetricTon => 1e6,
            Megagram => 1e6,
            Gigagram => 1e9,
            Teragram => 1e12,
            Petagram => 1e15,
            Exagram => 1e18,
            Zettagram => 1e21,
            Yottagram => 1e24,
            Ounce => 28.3495,
            Pound => 453.592,
            Stone => 6350.29,
            USShortTon => 907184.74,
            ImperialTon => 1016046.91,
            Carat => 0.2,
            Grain => 0.0647989,
            Dram => 1.77185,
            Cwt => 50802.345,
            Pennyweight => 1.55517,
            TroyOunce => 31.1035,
            Slug => 14593.9,
            Obol => 0.72,
            Scruple => 1.29598,
            Tola => 11.66,
            Baht => 15.0,
            Momme => 3.75,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: WeightUnit = from_unit.parse()?;
    let to: WeightUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round() / 10000.0)
}
