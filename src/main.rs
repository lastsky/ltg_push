#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::env;

mod error;
mod config;
use config::Config;


fn main() {
    let cfg = Config::from_file(&config_filename());
    println!("{:?}", cfg);
}

fn config_filename() -> String {
    match env::args().nth(1) {
        Some(filename) => filename,
        None => "config.yml".to_string(),
    }
}
