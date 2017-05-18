use hyper;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use url::Url;
use serde_json;
use std::io::prelude::*;
use std::sync::Arc;

use config::TelegramConfig;
use error::Error;


#[derive(Clone)]
pub struct Telegram {
    client: Arc<hyper::Client>,
    config: TelegramConfig,
}
impl Telegram {
    pub fn new(config: TelegramConfig) -> Result<Telegram, Error> {
        let ssl = NativeTlsClient::new()?;
        let connector = HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);

        Ok(Telegram {
            client: Arc::new(client),
            config: config,
        })
    }
    pub fn send(&self, message: String) -> Result<(), Error> {
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
    pub fn chat_id(&self) -> Result<i32, Error> {
        let turl = format!("https://api.telegram.org/bot{}/getUpdates", self.config.bot);

        let mut response = self.client.get(&turl).send()?;
        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;
        let mut response: TelegramGetID = serde_json::from_str(&buffer)?;

        if !response.ok {
            return Err(Error::Text(format!("Telegram response not ok: {}", buffer)));
        }

        let id = match response.result.pop() {
            Some(result) => result.message.chat.id,
            None => {
                return Err(Error::Text(format!("could not get result from response: {}", buffer)))
            }
        };

        Ok(id)
    }
}

#[derive(Deserialize)]
struct TelegramResponse {
    ok: bool,
}

#[derive(Deserialize)]
struct TelegramGetID {
    ok: bool,
    result: Vec<TelegramGetIDResult>,
}
#[derive(Deserialize)]
struct TelegramGetIDResult {
    message: TelegramGetIDMessage,
}
#[derive(Deserialize)]
struct TelegramGetIDMessage {
    chat: TelegramGetIDChat,
}
#[derive(Deserialize)]
struct TelegramGetIDChat {
    id: i32,
}
