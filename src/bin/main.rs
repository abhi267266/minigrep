use std::env;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    

    //The bodies of the if let and the unwrap_or_else functions are the same in both cases: we print the error and exit.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

