use std::env;
use std::process;

use rust_solution::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problems parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_solution::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
