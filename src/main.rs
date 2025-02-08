use std::{env, process};

use cnv::Cmd;

fn main() {
    let args = env::args();
    let cmd = Cmd::new(args).unwrap_or_else(|e| {
        println!("invalid arguments - {}", e);
        process::exit(1)
    });
    println!(
        "converting {} {} to {}",
        cmd.value, cmd.from_unit, cmd.to_unit
    );
}
