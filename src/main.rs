extern crate nom;

use std::io::{Write, BufRead};
use std::env;

mod time_math;

fn main() {
    let args = env::args();
    if args.len() > 1 {
        for argument in args.skip(1) {
            calculate(argument.as_str());
        }
    } else {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let stdin = std::io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();

        calculate(line.as_str());
    }
}

fn calculate(line: &str) {
    let result = time_math::exec(line);

    match result {
        Ok(d) => println!("{}", d),
        Err(e) => eprintln!("Err: {:#?}", e)
    }
}
