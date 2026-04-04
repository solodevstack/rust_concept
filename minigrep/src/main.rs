use std::env;
use std::process;

// added this just to fix intellisense autocomplete
extern crate minigrep;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // println!("{:#?}", args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsins arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    // The run function doesn’t return a value that we want to unwrap
    // in the same way that Config::new returns the Config instance.
    // Because run returns () in the success case, we only care about detecting
    // an error, so we don’t need unwrap_or_else to return the unwrapped value
    // because it would only be ()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
