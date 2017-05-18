use regex::Regex;
use std::collections::HashMap;

use config::FileConfig;
use error::Error;


pub struct Matcher {
    files: HashMap<String, Regex>,
}
impl Matcher {
    pub fn new(files: Vec<FileConfig>) -> Result<Matcher, Error> {
        let mut map = HashMap::new();

        for file in files {
            match file.regex {
                Some(r) => {
                    map.insert(file.path, Regex::new(&r)?);
                }
                None => {}
            };
        }

        Ok(Matcher { files: map })
    }
    pub fn is_matches(&self, filepath: &str, diff: String) -> bool {
        match self.files.get(filepath) {
            Some(r) => r.is_match(&diff),
            None => true,
        }
    }
}
