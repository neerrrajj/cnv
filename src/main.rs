use clap::Parser;

use cnv::Cmd;

fn main() {
    let cmd = Cmd::parse();
    
    match cmd.execute() {
        Ok((value, from_unit, result, to_unit)) => println!("{} {} = {} {}", value, from_unit, result, to_unit),
        Err(e) => println!("Unexpected error - {}" ,e)
    }
}
