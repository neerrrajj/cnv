use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum TimeUnit {
    Yoctosecond,
    Zeptosecond,
    Attosecond,
    Femtosecond,
    Picosecond,
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Fortnight,
    Month,
    Year,
    Decade,
    Century,
    Millennium,
    Megayear,
    Gigayear,
    Terayear,
    Eon,
}

pub struct UnitDef {
    variant: TimeUnit,
    name: &'static str,
    aliases: &'static [&'static str],
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef { variant: TimeUnit::Yoctosecond, name: "Yoctosecond", aliases: &["ys", "yocto", "yoctosec", "yoctosecond", "yoctoseconds"] },
    UnitDef { variant: TimeUnit::Zeptosecond, name: "Zeptosecond", aliases: &["zs", "zepto", "zeptosec", "zeptosecond", "zeptoseconds"] },
    UnitDef { variant: TimeUnit::Attosecond, name: "Attosecond", aliases: &["as", "atto", "attosec", "attosecond", "attoseconds"] },
    UnitDef { variant: TimeUnit::Femtosecond, name: "Femtosecond", aliases: &["fs", "femto", "femtosec", "femtosecond", "femtoseconds"] },
    UnitDef { variant: TimeUnit::Picosecond, name: "Picosecond", aliases: &["ps", "pico", "picosec", "picosecond", "picoseconds"] },
    UnitDef { variant: TimeUnit::Nanosecond, name: "Nanosecond", aliases: &["ns", "nano", "nanosec", "nanosecond", "nanoseconds"] },
    UnitDef { variant: TimeUnit::Microsecond, name: "Microsecond", aliases: &["µs", "us", "micro", "microsec", "usec", "microsecond", "microseconds"] },
    UnitDef { variant: TimeUnit::Millisecond, name: "Millisecond", aliases: &["ms", "milli", "millisec", "millisecond", "milliseconds"] },
    UnitDef { variant: TimeUnit::Second, name: "Second", aliases: &["s", "sec", "secs", "second", "seconds"] },
    UnitDef { variant: TimeUnit::Minute, name: "Minute", aliases: &["min", "mins", "minute", "minutes"] },
    UnitDef { variant: TimeUnit::Hour, name: "Hour", aliases: &["h", "hr", "hrs", "hour", "hours"] },
    UnitDef { variant: TimeUnit::Day, name: "Day", aliases: &["d", "dy", "day", "days"] },
    UnitDef { variant: TimeUnit::Week, name: "Week", aliases: &["w", "wk", "wks", "week", "weeks"] },
    UnitDef { variant: TimeUnit::Fortnight, name: "Fortnight", aliases: &["fortnight", "fortnights", "two weeks"] },
    UnitDef { variant: TimeUnit::Month, name: "Month", aliases: &["mo", "mnth", "month", "months"] },
    UnitDef { variant: TimeUnit::Year, name: "Year", aliases: &["y", "yr", "yrs", "year", "years"] },
    UnitDef { variant: TimeUnit::Decade, name: "Decade", aliases: &["dec", "decade", "decades", "10 years"] },
    UnitDef { variant: TimeUnit::Century, name: "Century", aliases: &["cent", "century", "centuries", "100 years"] },
    UnitDef { variant: TimeUnit::Millennium, name: "Millennium", aliases: &["millennium", "millennia", "1000 years"] },
    UnitDef { variant: TimeUnit::Megayear, name: "Megayear", aliases: &["Ma", "megayear", "megayears", "million years"] },
    UnitDef { variant: TimeUnit::Gigayear, name: "Gigayear", aliases: &["Ga", "gigayear", "gigayears", "billion years"] },
    UnitDef { variant: TimeUnit::Terayear, name: "Terayear", aliases: &["Ta", "terayear", "terayears", "trillion years"] },
    UnitDef { variant: TimeUnit::Eon, name: "Eon", aliases: &["eon", "eons", "aeon", "aeons"] },
];

impl_conversion_traits!(TimeUnit, UNIT_DEFS);

pub fn help_text() -> String {
    TimeUnit::generate_help_text()
}

impl TimeUnit {
    fn factor(&self) -> f64 {
        use TimeUnit::*;
        
        match self {
            Yoctosecond => 1e-24,
            Zeptosecond => 1e-21,
            Attosecond => 1e-18,
            Femtosecond => 1e-15,
            Picosecond => 1e-12,
            Nanosecond => 1e-9,
            Microsecond => 1e-6,
            Millisecond => 1e-3,
            Second => 1.0,
            Minute => 60.0,
            Hour => 60.0 * 60.0,
            Day => 24.0 * 60.0 * 60.0,
            Week => 7.0 * 24.0 * 60.0 * 60.0,
            Fortnight => 14.0 * 24.0 * 60.0 * 60.0,
            Month => 30.4375 * 24.0 * 60.0 * 60.0,
            Year => 365.25 * 24.0 * 60.0 * 60.0,
            Decade => 10.0 * 365.25 * 24.0 * 60.0 * 60.0,
            Century => 100.0 * 365.25 * 24.0 * 60.0 * 60.0,
            Millennium => 1000.0 * 365.25 * 24.0 * 60.0 * 60.0,
            Megayear => 1e6 * 365.25 * 24.0 * 60.0 * 60.0,
            Gigayear => 1e9 * 365.25 * 24.0 * 60.0 * 60.0,
            Terayear => 1e12 * 365.25 * 24.0 * 60.0 * 60.0,
            Eon => 500e6 * 365.25 * 24.0 * 60.0 * 60.0,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: TimeUnit = from_unit.parse()?;
    let to: TimeUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 10000.0).round() / 10000.0)
}
