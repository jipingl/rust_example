use mini_grep::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for [{}] in file [{}]",
        config.query, config.file_path
    );

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
