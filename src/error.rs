use serde_yaml;
use notify;
use hyper;
use hyper_native_tls::native_tls;
use url;
use serde_json;
use regex;
use std::io;


#[derive(Debug)]
pub enum Error {
    Text(String),
    Io(io::Error),
    ParseYaml(serde_yaml::Error),
    ParseJson(serde_json::Error),
    INotify(notify::Error),
    Hyper(hyper::Error),
    Tls(native_tls::Error),
    Url(url::ParseError),
    Regex(regex::Error),
}
impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Text(e)
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}
impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Error::ParseYaml(e)
    }
}
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::ParseJson(e)
    }
}
impl From<notify::Error> for Error {
    fn from(e: notify::Error) -> Self {
        Error::INotify(e)
    }
}
impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}
impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Error::Tls(e)
    }
}
impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::Url(e)
    }
}
impl From<regex::Error> for Error {
    fn from(e: regex::Error) -> Self {
        Error::Regex(e)
    }
}
