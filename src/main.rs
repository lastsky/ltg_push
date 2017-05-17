#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate notify;
extern crate hyper;
extern crate hyper_native_tls;
extern crate url;

use std::env;

mod error;
mod config;
mod watcher;
mod telegram;
use config::Config;
use watcher::LogWatcher;
use telegram::Telegram;


fn main() {
    let cfg = Config::from_file(&config_filename()).unwrap();
    let tg = Telegram::new(cfg.telegram).unwrap();
    let mut lw = LogWatcher::new(cfg.files, tg).unwrap();
    lw.watch().unwrap();
}

fn config_filename() -> String {
    match env::args().nth(1) {
        Some(filename) => filename,
        None => "config.yml".to_string(),
    }
}
