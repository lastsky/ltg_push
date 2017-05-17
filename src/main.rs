#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate notify;

use std::env;

mod error;
mod config;
mod watcher;
use config::Config;
use watcher::LogWatcher;


fn main() {
    let cfg = Config::from_file(&config_filename()).unwrap();
    let mut lw = LogWatcher::new(cfg.files).unwrap();
    lw.watch().unwrap();
}

fn config_filename() -> String {
    match env::args().nth(1) {
        Some(filename) => filename,
        None => "config.yml".to_string(),
    }
}
