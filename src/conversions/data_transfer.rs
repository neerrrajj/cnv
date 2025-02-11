use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum DataTransferUnit {
    BitPerSecond,
    BytePerSecond,
    KilobitPerSecond,
    KibibitPerSecond,
    KilobytePerSecond,
    KibibytePerSecond,
    MegabitPerSecond,
    MebibitPerSecond,
    MegabytePerSecond,
    MebibytePerSecond,
    GigabitPerSecond,
    GibibitPerSecond,
    GigabytePerSecond,
    GibibytePerSecond,
    TerabitPerSecond,
    TebibitPerSecond,
    TerabytePerSecond,
    TebibytePerSecond,
    PetabitPerSecond,
    PebibitPerSecond,
    PetabytePerSecond,
    PebibytePerSecond,
    ExabitPerSecond,
    ExbibitPerSecond,
    ExabytePerSecond,
    ExbibytePerSecond,
    ZettabitPerSecond,
    ZebibitPerSecond,
    ZettabytePerSecond,
    ZebibytePerSecond,
    YottabitPerSecond,
    YobibitPerSecond,
    YottabytePerSecond,
    YobibytePerSecond,
}

pub struct UnitDef {
    variant: DataTransferUnit,
    name: &'static str,
    aliases: &'static [&'static str],
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef { variant: DataTransferUnit::BitPerSecond, name: "Bit/s", aliases: &["bps", "bit/s"] },
    UnitDef { variant: DataTransferUnit::BytePerSecond, name: "Byte/s", aliases: &["Bps", "byte/s"] },
    UnitDef { variant: DataTransferUnit::KilobitPerSecond, name: "Kilobit/s", aliases: &["Kbps", "kbps", "kilobit/s"] },
    UnitDef { variant: DataTransferUnit::KilobytePerSecond, name: "Kilobyte/s", aliases: &["KBps", "KB/s", "kilobyte/s"] },
    UnitDef { variant: DataTransferUnit::MegabitPerSecond, name: "Megabit/s", aliases: &["Mbps", "mbps", "megabit/s"] },
    UnitDef { variant: DataTransferUnit::MegabytePerSecond, name: "Megabyte/s", aliases: &["MBps", "MB/s", "megabyte/s"] },
    UnitDef { variant: DataTransferUnit::GigabitPerSecond, name: "Gigabit/s", aliases: &["Gbps", "gbps", "gigabit/s"] },
    UnitDef { variant: DataTransferUnit::GigabytePerSecond, name: "Gigabyte/s", aliases: &["GBps", "GB/s", "gigabyte/s"] },
    UnitDef { variant: DataTransferUnit::TerabitPerSecond, name: "Terabit/s", aliases: &["Tbps", "tbps", "terabit/s"] },
    UnitDef { variant: DataTransferUnit::TerabytePerSecond, name: "Terabyte/s", aliases: &["TBps", "TB/s", "terabyte/s"] },
    UnitDef { variant: DataTransferUnit::PetabitPerSecond, name: "Petabit/s", aliases: &["Pbps", "pbps", "petabit/s"] },
    UnitDef { variant: DataTransferUnit::PetabytePerSecond, name: "Petabyte/s", aliases: &["PBps", "PB/s", "petabyte/s"] },
    UnitDef { variant: DataTransferUnit::ExabitPerSecond, name: "Exabit/s", aliases: &["Ebps", "ebps", "exabit/s"] },
    UnitDef { variant: DataTransferUnit::ExabytePerSecond, name: "Exabyte/s", aliases: &["EBps", "EB/s", "exabyte/s"] },
    UnitDef { variant: DataTransferUnit::ZettabitPerSecond, name: "Zettabit/s", aliases: &["Zbps", "zbps", "zettabit/s"] },
    UnitDef { variant: DataTransferUnit::ZettabytePerSecond, name: "Zettabyte/s", aliases: &["ZBps", "ZB/s", "zettabyte/s"] },
    UnitDef { variant: DataTransferUnit::YottabitPerSecond, name: "Yottabit/s", aliases: &["Ybps", "ybps", "yottabit/s"] },
    UnitDef { variant: DataTransferUnit::YottabytePerSecond, name: "Yottabyte/s", aliases: &["YBps", "YB/s", "yottabyte/s"] },
    UnitDef { variant: DataTransferUnit::KibibitPerSecond, name: "Kibibit/s", aliases: &["Kibps", "Kibit/s"] },
    UnitDef { variant: DataTransferUnit::KibibytePerSecond, name: "Kibibyte/s", aliases: &["KiBps", "KiB/s"] },
    UnitDef { variant: DataTransferUnit::MebibitPerSecond, name: "Mebibit/s", aliases: &["Mibps", "Mibit/s"] },
    UnitDef { variant: DataTransferUnit::MebibytePerSecond, name: "Mebibyte/s", aliases: &["MiBps", "MiB/s"] },
    UnitDef { variant: DataTransferUnit::GibibitPerSecond, name: "Gibibit/s", aliases: &["Gibps", "Gibit/s"] },
    UnitDef { variant: DataTransferUnit::GibibytePerSecond, name: "Gibibyte/s", aliases: &["GiBps", "GiB/s"] },
    UnitDef { variant: DataTransferUnit::TebibitPerSecond, name: "Tebibit/s", aliases: &["Tibps", "Tibit/s"] },
    UnitDef { variant: DataTransferUnit::TebibytePerSecond, name: "Tebibyte/s", aliases: &["TiBps", "TiB/s"] },
    UnitDef { variant: DataTransferUnit::PebibitPerSecond, name: "Pebibit/s", aliases: &["Pibps", "Pibit/s"] },
    UnitDef { variant: DataTransferUnit::PebibytePerSecond, name: "Pebibyte/s", aliases: &["PiBps", "PiB/s"] },
    UnitDef { variant: DataTransferUnit::ExbibitPerSecond, name: "Exbibit/s", aliases: &["Eibps", "Eibit/s"] },
    UnitDef { variant: DataTransferUnit::ExbibytePerSecond, name: "Exbibyte/s", aliases: &["EiBps", "EiB/s"] },
    UnitDef { variant: DataTransferUnit::ZebibitPerSecond, name: "Zebibit/s", aliases: &["Zibps", "Zibit/s"] },
    UnitDef { variant: DataTransferUnit::ZebibytePerSecond, name: "Zebibyte/s", aliases: &["ZiBps", "ZiB/s"] },
    UnitDef { variant: DataTransferUnit::YobibitPerSecond, name: "Yobibit/s", aliases: &["Yibps", "Yibit/s"] },
    UnitDef { variant: DataTransferUnit::YobibytePerSecond, name: "Yobibyte/s", aliases: &["YiBps", "YiB/s"] },
];

impl_conversion_traits!(DataTransferUnit, UNIT_DEFS);

pub fn help_text() -> String {
    DataTransferUnit::generate_help_text()
}

impl DataTransferUnit {
    fn factor(&self) -> f64 {
        use DataTransferUnit::*;

        match self {
            BitPerSecond => 1.0,
            BytePerSecond => 8.0,
            KilobitPerSecond => 1_000.0,
            KilobytePerSecond => 8_000.0,
            MegabitPerSecond => 1_000_000.0,
            MegabytePerSecond => 8_000_000.0,
            GigabitPerSecond => 1_000_000_000.0,
            GigabytePerSecond => 8_000_000_000.0,
            TerabitPerSecond => 1e12,
            TerabytePerSecond => 8e12,
            PetabitPerSecond => 1e15,
            PetabytePerSecond => 8e15,
            ExabitPerSecond => 1e18,
            ExabytePerSecond => 8e18,
            ZettabitPerSecond => 1e21,
            ZettabytePerSecond => 8e21,
            YottabitPerSecond => 1e24,
            YottabytePerSecond => 8e24,
            KibibitPerSecond => 1024.0,
            KibibytePerSecond => 1024.0 * 8.0,
            MebibitPerSecond => 1_048_576.0,
            MebibytePerSecond => 1_048_576.0 * 8.0,
            GibibitPerSecond => 1_073_741_824.0,
            GibibytePerSecond => 1_073_741_824.0 * 8.0,
            TebibitPerSecond => 1_099_511_627_776.0,
            TebibytePerSecond => 1_099_511_627_776.0 * 8.0,
            PebibitPerSecond => 1_125_899_906_842_624.0,
            PebibytePerSecond => 1_125_899_906_842_624.0 * 8.0,
            ExbibitPerSecond => 1_152_921_504_606_846_976.0,
            ExbibytePerSecond => 1_152_921_504_606_846_976.0 * 8.0,
            ZebibitPerSecond => 1_180_591_620_717_411_303_424.0,
            ZebibytePerSecond => 1_180_591_620_717_411_303_424.0 * 8.0,
            YobibitPerSecond => 1_208_925_819_614_629_174_706_176.0,
            YobibytePerSecond => 1_208_925_819_614_629_174_706_176.0 * 8.0,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: DataTransferUnit = from_unit.parse()?;
    let to: DataTransferUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round() / 10000.0)
}
