use clap::Parser;

use cnv::Cmd;

fn main() {
    let cmd = Cmd::parse();

    match cmd.execute() {
        Ok((value, from_unit, result, to_unit, date_opt)) => {
            let mut output = String::new();

            let conversion_line = format!("{} {} = {} {}", value, from_unit, result, to_unit);
            let dashes = "-".repeat(conversion_line.len() + 1);

            output.push_str(&format!("{}\n{}\n{}", dashes, conversion_line, dashes));

            if let Some(date) = date_opt {
                output.push_str(&format!("\nas of: {}", date));
            }

            println!("{}", output);
        }
        Err(e) => println!("{}", e),
    }
}
