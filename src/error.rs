use std::io;
use serde_yaml;


#[derive(Debug)]
pub enum Error {
    Text(String),
    Io(io::Error),
    ParseYaml(serde_yaml::Error),
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
