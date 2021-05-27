use std::env;
use std::process;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = lib::run(&args[1]) {
        eprintln!("error encountered: {}", e);
        process::exit(1);
    }
}
