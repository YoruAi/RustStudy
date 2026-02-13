use chapter12_io_project::Config;
use std::{env, process};

fn main() {
    println!("[project minigrep]");

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        // output to stderr
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = chapter12_io_project::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
