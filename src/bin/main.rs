use std::{env, process};
use minigrep::Config;


fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    //The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print the error and exit.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

