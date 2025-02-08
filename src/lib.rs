use std::env::Args;

#[derive(Debug)]
pub struct Cmd {
    pub unit: String,
    pub value: f64,
    pub from_unit: String,
    pub to_unit: String,
}

impl Cmd {
    pub fn new(mut args: Args) -> Result<Cmd, &'static str> {
        args.next();
        let unit = match args.next() {
            Some(v) => v,
            None => return Err("Unit type not supported. List of unit type supported: 'dist'"),
        };
        let value: f64 = match args.next() {
            Some(v) => v.parse().map_err(|_| "Invalid number")?,
            None => return Err("Unit not supported. List of units supported: 'dist'"),
        };
        let from_unit = match args.next() {
            Some(v) => v,
            None => return Err("From unit not supported. List of from units supported: 'dist'"),
        };
        let to_unit = match args.next() {
            Some(v) => v,
            None => return Err("To unit not supported. List of to units supported: 'dist'"),
        };
        Ok(Cmd {
            unit,
            value,
            from_unit,
            to_unit,
        })
    }
}
