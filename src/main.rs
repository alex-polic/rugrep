use std::env::args;
use std::process;
use lib::Config;

mod lib;

fn main() {
    let args : Vec<String> = args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(
        |err| {
            eprintln!("Error occurred while building config {}", err);
            process::exit(1);
        }
    );

    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
