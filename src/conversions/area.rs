use std::f64::consts::PI;
use std::str::FromStr;

use super::*;

#[derive(Debug, Clone, Copy)]
pub enum AreaUnit {
    SquareKilometer,
    SquareMeter,
    SquareCentimeter,
    SquareMillimeter,
    Hectare,
    Are,
    Decare,
    SquareMile,
    AcreInternational,
    AcreUSSurvey,
    SquareYard,
    SquareFootInternational,
    SquareFootUSSurvey,
    SquareInch,
    Rood,
    SquareChain,
    SquareFurlong,
    SquarePerch,
    Township,
    Section,
    Homestead,
    Dunam,
    Tsubo,
    Ping,
    CircularInch,
    CircularMil,
    Barn,
    Centiare,
    Deciare,
    Killa,
    Ground,
    Cent,
}

pub struct UnitDef {
    variant: AreaUnit,
    name: &'static str,
    aliases: &'static [&'static str],
}

const UNIT_DEFS: &[UnitDef] = &[
    UnitDef {
        variant: AreaUnit::SquareKilometer,
        name: "Square Kilometer",
        aliases: &[
            "km2", "sqkm", "sq km", "square km",
            "square kilometer", "square kilometers", "square kilometre", "square kilometres",
            "sq kilometer", "sq kilometre", "sq kilometers", "sq kilometres",
            "kilometers squared", "kilometres squared",
        ],
    },
    UnitDef {
        variant: AreaUnit::SquareMeter,
        name: "Square Meter",
        aliases: &[
            "m2", "sqm", "sq m", "square m",
            "square meter", "square meters", "square metre", "square metres",
            "sq meter", "sq metre", "sq meters", "sq metres",
            "meters squared", "metres squared",
        ],
    },
    UnitDef {
        variant: AreaUnit::SquareCentimeter,
        name: "Square Centimeter",
        aliases: &[
            "cm2", "sqcm", "sq cm", "square cm",
            "square centimeter", "square centimeters", "square centimetre", "square centimetres",
            "sq centimeter", "sq centimetre", "sq centimeters", "sq centimetres",
            "centimeters squared", "centimetres squared",
        ],
    },
    UnitDef {
        variant: AreaUnit::SquareMillimeter,
        name: "Square Millimeter",
        aliases: &[
            "mm2", "sqmm", "sq mm", "square mm",
            "square millimeter", "square millimeters", "square millimetre", "square millimetres",
            "sq millimeter", "sq millimetre", "sq millimeters", "sq millimetres",
            "millimeters squared", "millimetres squared",
        ],
    },
    UnitDef {
        variant: AreaUnit::Hectare,
        name: "Hectare",
        aliases: &["ha", "hectare", "hectares", "hectar"],
    },
    UnitDef {
        variant: AreaUnit::Are,
        name: "Are",
        aliases: &[
            "a", "are", "ares", "sq dam", "square decametre", "square decametres",
            "square decameter", "square decameters",
        ],
    },
    UnitDef {
        variant: AreaUnit::Decare,
        name: "Decare",
        aliases: &[ 
            "daa", "decare", "decares", "sq hm", "square hectometre", "square hectometres",
            "square hectometer", "square hectometers",
        ],
    },
    UnitDef {
        variant: AreaUnit::SquareMile,
        name: "Square Mile",
        aliases: &["sqmi", "square mi", "square mile", "square miles", "miles squared"],
    },
    UnitDef {
        variant: AreaUnit::AcreInternational,
        name: "Acre (International)",
        aliases: &["ac", "acre", "acres", "international acre"],
    },
    UnitDef {
        variant: AreaUnit::AcreUSSurvey,
        name: "Acre (US Survey)",
        aliases: &["us ac", "survey acre", "us acre"],
    },
    UnitDef {
        variant: AreaUnit::SquareYard,
        name: "Square Yard",
        aliases: &["yd2", "sqyd", "sq yd", "square yd", "square yard", "square yards", "yards squared"],
    },
    UnitDef {
        variant: AreaUnit::SquareFootInternational,
        name: "Square Foot (International)",
        aliases: &[
            "ft2", "sqft", "square ft", "square foot", "square feet",
            "sq ft", "feet squared", "foot squared",
        ],
    },
    UnitDef {
        variant: AreaUnit::SquareFootUSSurvey,
        name: "Square Foot (US Survey)",
        aliases: &["us square foot", "survey square foot"],
    },
    UnitDef {
        variant: AreaUnit::SquareInch,
        name: "Square Inch",
        aliases: &["in2", "sqin", "sq in", "square in", "square inch", "square inches", "inches squared"],
    },
    UnitDef {
        variant: AreaUnit::Rood,
        name: "Rood",
        aliases: &["rood", "roods"],
    },
    UnitDef {
        variant: AreaUnit::SquareChain,
        name: "Square Chain",
        aliases: &[
            "sqch", "sq ch", "square chain", "square chains", "chains squared",
        ],
    },
    UnitDef {
        variant: AreaUnit::SquareFurlong,
        name: "Square Furlong",
        aliases: &["square furlong", "square furlongs", "furlongs squared"],
    },
    UnitDef {
        variant: AreaUnit::SquarePerch,
        name: "Square Perch",
        aliases: &[
            "square perch", "square perches", "square rod", "square rods",
            "square pole", "square poles", "sq rod", "sq pole", "sq perch", 
        ],
    },
    UnitDef {
        variant: AreaUnit::Township,
        name: "Township",
        aliases: &["township", "townships", "twp"],
    },
    UnitDef {
        variant: AreaUnit::Section,
        name: "Section",
        aliases: &["section", "sections"],
    },
    UnitDef {
        variant: AreaUnit::Homestead,
        name: "Homestead",
        aliases: &["homestead", "homesteads"],
    },
    UnitDef {
        variant: AreaUnit::Dunam,
        name: "Dunam",
        aliases: &["dunam", "dunams", "donum", "dunum"],
    },
    UnitDef {
        variant: AreaUnit::Tsubo,
        name: "Tsubo",
        aliases: &["tsubo", "tsubos"],
    },
    UnitDef {
        variant: AreaUnit::Ping,
        name: "Ping",
        aliases: &["ping", "pings"],
    },
    UnitDef {
        variant: AreaUnit::CircularInch,
        name: "Circular Inch",
        aliases: &["circular inch", "circular inches", "circ in", "cin"],
    },
    UnitDef {
        variant: AreaUnit::CircularMil,
        name: "Circular Mil",
        aliases: &["circular mil", "circular mils", "cmil"],
    },
    UnitDef {
        variant: AreaUnit::Barn,
        name: "Barn",
        aliases: &["barn", "barns", "b"],
    },
    UnitDef {
        variant: AreaUnit::Centiare,
        name: "Centiare",
        aliases: &["centiare", "centiares", "ca"],
    },
    UnitDef {
        variant: AreaUnit::Deciare,
        name: "Deciare",
        aliases: &["deciare", "deciares", "da"],
    },
    UnitDef {
        variant: AreaUnit::Killa,
        name: "Killa",
        aliases: &["killa"],
    },
    UnitDef {
        variant: AreaUnit::Ground,
        name: "Ground",
        aliases: &["ground"],
    },
    UnitDef {
        variant: AreaUnit::Cent,
        name: "Cent",
        aliases: &["cent"],
    },
];

impl_conversion_traits!(AreaUnit, UNIT_DEFS);

pub fn help_text() -> String {
    AreaUnit::generate_help_text()
}

impl AreaUnit {
    fn factor(&self) -> f64 {
        use AreaUnit::*;
        
        match self {
            SquareKilometer => 1e6,
            SquareMeter => 1.0,
            SquareCentimeter => 1e-4,
            SquareMillimeter => 1e-6,
            Hectare => 1e4,
            Are => 100.0,
            Decare => 1e3,
            SquareMile => 2.589988110336e6,
            AcreInternational => 4046.8564224,
            AcreUSSurvey => 4046.8726098,
            SquareYard => 0.83612736,
            SquareFootInternational => 0.09290304,
            SquareFootUSSurvey => 0.0929034116132,
            SquareInch => 0.00064516,
            Rood => 1011.7141056,
            SquareChain => 404.68564224,
            SquareFurlong => 40468.564224,
            SquarePerch => 25.29285264,
            Township => 93.23957197215e6,
            Section => 2.589988110336e6,
            Homestead => 4046.8564224 * 160.0,
            Dunam => 1e3,
            Tsubo => 3.305785,
            Ping => 3.3058,
            CircularInch => PI * (0.0127_f64).powi(2),
            CircularMil => PI * (0.0000127_f64).powi(2),
            Barn => 1e-28,
            Centiare => 1.0,
            Deciare => 10.0,
            Killa => 4046.86,
            Ground => 222.967,
            Cent => 40.4686,
        }
    }
}

pub fn convert(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, &'static str> {
    let from: AreaUnit = from_unit.parse()?;
    let to: AreaUnit = to_unit.parse()?;

    let result = value * from.factor() / to.factor();
    Ok((result * 1e9).round() / 1e9)
}