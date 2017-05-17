use std::io;
use serde_yaml;
use notify;


#[derive(Debug)]
pub enum Error {
    Text(String),
    Io(io::Error),
    ParseYaml(serde_yaml::Error),
    INotify(notify::Error),
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
impl From<notify::Error> for Error {
    fn from(e: notify::Error) -> Self {
        Error::INotify(e)
    }
}
