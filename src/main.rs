use clap::Parser;

use cnv::Cmd;

fn main() {
    let cmd = Cmd::parse();
    
    match cmd.execute() {
        Ok((value, from_unit, result, to_unit)) => {
            let output = format!("{} {} = {} {}", value, from_unit, result, to_unit);
            let dashes = "-".repeat(output.len()+1);
            println!("\n{}\n{}\n{}\n", dashes, output, dashes);
            }
        Err(e) => println!("Unexpected error - {}\n" ,e)
    }
}
