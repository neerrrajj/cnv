use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum DataStorageUnit {
    Bit,
    Byte,
    Kilobit,
    Kibibit,
    Kilobyte,
    Kibibyte,
    Megabit,
    Mebibit,
    Megabyte,
    Mebibyte,
    Gigabit,
    Gibibit,
    Gigabyte,
    Gibibyte,
    Terabit,
    Tebibit,
    Terabyte,
    Tebibyte,
    Petabit,
    Pebibit,
    Petabyte,
    Pebibyte,
    Exabit,
    Exbibit,
    Exabyte,
    Exbibyte,
    Zettabit,
    Zebibit,
    Zettabyte,
    Zebibyte,
    Yottabit,
    Yobibit,
    Yottabyte,
    Yobibyte,
}

pub struct UnitDef {
    variant: DataStorageUnit,
    name: &'static str,
    aliases: &'static [&'static str],
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef { variant: DataStorageUnit::Bit, name: "Bit", aliases: &["b", "bit", "bits"] },
    UnitDef { variant: DataStorageUnit::Byte, name: "Byte", aliases: &["B", "byte", "bytes"] },
    UnitDef { variant: DataStorageUnit::Kilobit, name: "Kilobit", aliases: &["Kb", "kilobit"] },
    UnitDef { variant: DataStorageUnit::Kilobyte, name: "Kilobyte", aliases: &["KB", "kilobyte"] },
    UnitDef { variant: DataStorageUnit::Megabit, name: "Megabit", aliases: &["Mb", "megabit"] },
    UnitDef { variant: DataStorageUnit::Megabyte, name: "Megabyte", aliases: &["MB", "megabyte"] },
    UnitDef { variant: DataStorageUnit::Gigabit, name: "Gigabit", aliases: &["Gb", "gigabit"] },
    UnitDef { variant: DataStorageUnit::Gigabyte, name: "Gigabyte", aliases: &["GB", "gigabyte"] },
    UnitDef { variant: DataStorageUnit::Terabit, name: "Terabit", aliases: &["Tb", "terabit"] },
    UnitDef { variant: DataStorageUnit::Terabyte, name: "Terabyte", aliases: &["TB", "terabyte"] },
    UnitDef { variant: DataStorageUnit::Petabit, name: "Petabit", aliases: &["Pb", "petabit"] },
    UnitDef { variant: DataStorageUnit::Petabyte, name: "Petabyte", aliases: &["PB", "petabyte"] },
    UnitDef { variant: DataStorageUnit::Exabit, name: "Exabit", aliases: &["Eb", "exabit"] },
    UnitDef { variant: DataStorageUnit::Exabyte, name: "Exabyte", aliases: &["EB", "exabyte"] },
    UnitDef { variant: DataStorageUnit::Zettabit, name: "Zettabit", aliases: &["Zb", "zettabit"] },
    UnitDef { variant: DataStorageUnit::Zettabyte, name: "Zettabyte", aliases: &["ZB", "zettabyte"] },
    UnitDef { variant: DataStorageUnit::Yottabit, name: "Yottabit", aliases: &["Yb", "yottabit"] },
    UnitDef { variant: DataStorageUnit::Yottabyte, name: "Yottabyte", aliases: &["YB", "yottabyte"] },
    UnitDef { variant: DataStorageUnit::Kibibit, name: "Kibibit", aliases: &["Kib", "kibibit"] },
    UnitDef { variant: DataStorageUnit::Kibibyte, name: "Kibibyte", aliases: &["KiB", "kibibyte"] },
    UnitDef { variant: DataStorageUnit::Mebibit, name: "Mebibit", aliases: &["Mib", "mebibit"] },
    UnitDef { variant: DataStorageUnit::Mebibyte, name: "Mebibyte", aliases: &["MiB", "mebibyte"] },
    UnitDef { variant: DataStorageUnit::Gibibit, name: "Gibibit", aliases: &["Gib", "gibibit"] },
    UnitDef { variant: DataStorageUnit::Gibibyte, name: "Gibibyte", aliases: &["GiB", "gibibyte"] },
    UnitDef { variant: DataStorageUnit::Tebibit, name: "Tebibit", aliases: &["Tib", "tebibit"] },
    UnitDef { variant: DataStorageUnit::Tebibyte, name: "Tebibyte", aliases: &["TiB", "tebibyte"] },
    UnitDef { variant: DataStorageUnit::Pebibit, name: "Pebibit", aliases: &["Pib", "pebibit"] },
    UnitDef { variant: DataStorageUnit::Pebibyte, name: "Pebibyte", aliases: &["PiB", "pebibyte"] },
    UnitDef { variant: DataStorageUnit::Exbibit, name: "Exbibit", aliases: &["Eib", "exbibit"] },
    UnitDef { variant: DataStorageUnit::Exbibyte, name: "Exbibyte", aliases: &["EiB", "exbibyte"] },
    UnitDef { variant: DataStorageUnit::Zebibit, name: "Zebibit", aliases: &["Zib", "zebibit"] },
    UnitDef { variant: DataStorageUnit::Zebibyte, name: "Zebibyte", aliases: &["ZiB", "zebibyte"] },
    UnitDef { variant: DataStorageUnit::Yobibit, name: "Yobibit", aliases: &["Yib", "yobibit"] },
    UnitDef { variant: DataStorageUnit::Yobibyte, name: "Yobibyte", aliases: &["YiB", "yobibyte"] },
];

impl_conversion_traits!(DataStorageUnit, UNIT_DEFS);

pub fn help_text() -> String {
    DataStorageUnit::generate_help_text()
}

impl DataStorageUnit {
    fn factor(&self) -> f64 {
        use DataStorageUnit::*;

        match self {
            Bit => 1.0,
            Byte => 8.0,
            Kilobit => 1_000.0,
            Kilobyte => 8_000.0,
            Megabit => 1_000_000.0,
            Megabyte => 8_000_000.0,
            Gigabit => 1_000_000_000.0,
            Gigabyte => 8_000_000_000.0,
            Terabit => 1e12,
            Terabyte => 8e12,
            Petabit => 1e15,
            Petabyte => 8e15,
            Exabit => 1e18,
            Exabyte => 8e18,
            Zettabit => 1e21,
            Zettabyte => 8e21,
            Yottabit => 1e24,
            Yottabyte => 8e24,
            Kibibit => 1024.0,
            Kibibyte => 8192.0,
            Mebibit => 1_048_576.0,
            Mebibyte => 8_388_608.0,
            Gibibit => 1_073_741_824.0,
            Gibibyte => 8_589_934_592.0,
            Tebibit => 1_099_511_627_776.0,
            Tebibyte => 8_796_093_022_208.0,
            Pebibit => 1_125_899_906_842_624.0,
            Pebibyte => 9_007_199_254_740_992.0,
            Exbibit => 1_152_921_504_606_846_976.0,
            Exbibyte => 9_223_372_036_854_775_808.0,
            Zebibit => 1_180_591_620_717_411_303_424.0,
            Zebibyte => 9_444_732_965_739_290_427_392.0,
            Yobibit => 1_208_925_819_614_629_174_706_176.0,
            Yobibyte => 9_671_406_556_917_033_397_649_408.0,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: DataStorageUnit = from_unit.parse()?;
    let to: DataStorageUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round()/10000.0)
}
