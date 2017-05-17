use hyper;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use url::Url;
use serde_json;
use std::io::prelude::*;

use config::TelegramConfig;
use error::Error;


pub struct Telegram {
    client: hyper::Client,
    config: TelegramConfig,
}
impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Telegram, Error> {
        let ssl = NativeTlsClient::new()?;
        let connector = HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);

        Ok(Telegram {
            client: client,
            config: config,
        })
    }
    pub fn send(&self, path: &str, diff: String) -> Result<(), Error> {
        let message = format!("**{}**\n\n{}", path, diff);

        let turl = format!("https://api.telegram.org/bot{}/sendMessage",
                           self.config.bot);
        let mut turl = Url::parse(&turl)?;

        turl.query_pairs_mut().append_pair("chat_id", &self.config.chat_id);
        turl.query_pairs_mut().append_pair("text", &message);
        turl.query_pairs_mut().append_pair("parse_mode", "markdown");

        let mut response = self.client.post(&turl.into_string()).send()?;
        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;
        let response: TelegramResponse = serde_json::from_str(&buffer)?;

        if !response.ok {
            return Err(Error::Text(format!("Telegram response not ok: {}", buffer)));
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct TelegramResponse {
    ok: bool,
}
