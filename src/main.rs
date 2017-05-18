#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate notify;
extern crate hyper;
extern crate hyper_native_tls;
extern crate url;
extern crate regex;

use std::env;
use std::process;

mod error;
mod config;
mod watcher;
mod telegram;
use config::Config;
use watcher::LogWatcher;
use telegram::Telegram;


fn main() {
    let cfg_filename = config_filename_or_die();
    let cfg = Config::from_file(&cfg_filename).unwrap();
    let tg = Telegram::new(cfg.telegram).unwrap();

    if is_get_chat_id() {
        let chat_id = tg.chat_id().unwrap();
        println!("Chat ID: {:?}", chat_id);
        return;
    };

    tg.send(format!("Started")).unwrap();
    let mut lw = match LogWatcher::new(cfg.files, tg.clone()) {
        Ok(lw) => lw,
        Err(e) => {
            tg.send(format!("*Internal error:* {:?}.\n\nStopped", e)).unwrap();
            return;
        }
    };
    match lw.watch() {
        Ok(()) => {}
        Err(e) => tg.send(format!("*Internal error:* {:?}.\n\nStopped", e)).unwrap(),
    };
}

fn config_filename_or_die() -> String {
    match env::args().nth(1) {
        Some(filename) => filename,
        None => {
            println!("Please provide config file");
            process::exit(2);
        }
    }
}

fn is_get_chat_id() -> bool {
    match env::args().nth(2) {
        Some(arg) => if arg == "getChatID" { true } else { false },
        None => false,
    }
}
