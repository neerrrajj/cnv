use std::str::FromStr;

#[derive(Debug)]
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

impl FromStr for WeightUnit {
    type Err = &'static str;

    fn from_str(unit: &str) -> Result<Self, Self::Err> {
        use WeightUnit::*;
        let unit = unit.to_lowercase();

        match unit.as_str() {
            "mcg" | "microgram" | "micrograms" => Ok(Microgram),
            "mg" | "milligram" | "milligrams" => Ok(Milligram),
            "g" | "gram" | "grams" => Ok(Gram),
            "kg" | "kgs" | "kilogram" | "kilograms" => Ok(Kilogram),
            "t" | "tonne" | "metricton" | "metrictons" => Ok(MetricTon),
            "Mg" | "megagram" | "megagrams" => Ok(Megagram),
            "Gg" | "gigagram" | "gigagrams" => Ok(Gigagram),
            "Tg" | "teragram" | "teragrams" => Ok(Teragram),
            "Pg" | "petagram" | "petagrams" => Ok(Petagram),
            "Eg" | "exagram" | "exagrams" => Ok(Exagram),
            "Zg" | "zettagram" | "zettagrams" => Ok(Zettagram),
            "Yg" | "yottagram" | "yottagrams" => Ok(Yottagram),
            "oz" | "ounce" | "ounces" => Ok(Ounce),
            "lb" | "lbs" | "pound" | "pounds" => Ok(Pound),
            "st" | "stone" | "stones" => Ok(Stone),
            "uston" | "usshortton" | "shortton" => Ok(USShortTon),
            "ukton" | "imperialton" | "longton" => Ok(ImperialTon),
            "ct" | "carat" | "carats" => Ok(Carat),
            "gr" | "grain" | "grains" => Ok(Grain),
            "dr" | "dram" | "drams" => Ok(Dram),
            "cwt" | "hundredweight" => Ok(Cwt),
            "dwt" | "pennyweight" | "pennyweights" => Ok(Pennyweight),
            "ozt" | "troyounce" | "troyounces" => Ok(TroyOunce),
            "slug" => Ok(Slug),
            "obol" => Ok(Obol),
            "scruple" => Ok(Scruple),
            "tola" => Ok(Tola),
            "baht" => Ok(Baht),
            "momme" => Ok(Momme),
            _ => Err("Invalid unit"),
        }
    }
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
