use std::fs::File;
use std::io::prelude::*;
use serde_yaml;

use error::Error;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub files: Vec<FileConfig>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FileConfig {
    pub path: String,
}
impl Config {
    pub fn from_file(filename: &str) -> Result<Config, Error> {
        let mut file = File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let config: Config = serde_yaml::from_str(&buffer)?;
        Ok(config)
    }
}
