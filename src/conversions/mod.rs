// conversions/mod.rs
pub mod distance;
pub mod weight;
pub mod temperature;

pub trait Help {
    fn generate_help_text() -> String;
}

macro_rules! impl_conversion_traits {
    ($type:ty, $unit_defs:ident) => {
        impl FromStr for $type {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let search = s.to_lowercase();
                $unit_defs
                    .iter()
                    .find(|def| def.aliases.contains(&search.as_str()))
                    .map(|def| def.variant)
                    .ok_or("Invalid unit")
            }
        }

        impl Help for $type {
            fn generate_help_text() -> String {
                let mut help = String::from("------------------------------\n\
                                             List of supported units\n\
                                             ------------------------------\n");
                for def in $unit_defs {
                    help.push_str(&format!("* {} : {}\n", 
                        def.name, 
                        def.aliases.join(", ")
                    ));
                }
                help
            }
        }
    };
}

pub(crate) use impl_conversion_traits;