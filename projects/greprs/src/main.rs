extern crate greprs;

use std::env;
use std::process;
use std::io::prelude::*;

use greprs::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
            writeln!(
                &mut stderr,
                "problem parsing arguments: {}",
                err
            ).expect("could not write to stderr");

            process::exit(1);
            });

    if let Err(e) = greprs::run(config) {
        writeln!(
            &mut stderr,
            "application error: {}",
            e
        ).expect("could not write to stderr");

        process::exit(1);
    }

}

