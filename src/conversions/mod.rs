pub mod distance;
pub mod weight;
pub mod temperature;
pub mod data_storage;
pub mod data_transfer;
pub mod time;
pub mod volume;

pub trait Help {
    fn generate_help_text() -> String;
}

macro_rules! impl_conversion_traits {
    ($type:ty, $unit_defs:ident) => {
        impl FromStr for $type {
            type Err = &'static str;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $unit_defs
                    .iter()
                    .find(|def| def.aliases.contains(&s))
                    .map(|def| def.variant)
                    .ok_or("Invalid unit.\nUse the --list (or) -L flag on a command to see the list of supported units\nUsage: cnv <COMMAND> --list")
            }
        }

        impl Help for $type {
            fn generate_help_text() -> String {
                let mut help = String::from("----------------------------\n\
                                             List of supported units\n\
                                             ----------------------------");
                for def in $unit_defs {
                    help.push_str(&format!("\n* {} : {}", 
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