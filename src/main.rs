use std::{env, process};

use cnv::Cmd;

fn main() {
    let args = env::args();
    let cmd = Cmd::new(args).unwrap_or_else(|e| {
        println!("Problem parsing arguments :/ \n{}", e);
        process::exit(1)
    });
    
    match cmd.execute() {
        Ok(v) => println!("{} {} = {} {}", cmd.value, cmd.from_unit, v, cmd.to_unit),
        Err(e) => println!("Unexpected error - {}" ,e)
    }
}
