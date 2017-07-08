extern crate rgrep;

use std::env;
use std::process;
use std::io::prelude::*;
use rgrep::Config;

fn main() {
    let mut stderr = std::io::stderr();
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        writeln!(
            &mut stderr,
            "Problem parsing parameters: {}",
            err
        ).expect("Could not write to stderr");
        process::exit(1);
    });
    if let Err(err) = rgrep::run(config) {
        writeln!(
            &mut stderr,
            "Application error: {}",
            err
        ).expect("Could not write to stderr");
        process::exit(1);
    }
}
