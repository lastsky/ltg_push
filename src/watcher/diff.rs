use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs;

use error::Error;
use config::FileConfig;


pub struct DiffFinder {
    files: HashMap<String, u64>,
}
impl DiffFinder {
    pub fn new(files: Vec<FileConfig>) -> Result<DiffFinder, Error> {
        let mut df = DiffFinder { files: HashMap::new() };
        df.init(files)?;

        Ok(df)
    }
    fn init(&mut self, files: Vec<FileConfig>) -> Result<(), Error> {
        for file in files {
            self.update_seek(file.path)?;
        }

        Ok(())
    }
    fn update_seek(&mut self, path: String) -> Result<(), Error> {
        let meta = fs::metadata(&path)?;
        if meta.is_dir() {
            return Err(Error::Text(format!("{} is dir (only file suported)", &path)));
        }
        self.files.insert(path, meta.len());
        Ok(())
    }
    pub fn find(&mut self, path: &str) -> Result<Option<String>, Error> {
        let seek = match self.files.get(path) {
            Some(seek) => *seek,
            None => return Err(Error::Text(format!("could not find file {} in diff finder", path))),
        };

        let new_len = fs::metadata(&path)?.len();
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        if seek < new_len {
            file.seek(SeekFrom::Start(seek))?;
        }

        file.read_to_end(&mut buffer)?;
        self.update_seek(path.to_owned())?;

        let buffer = String::from_utf8_lossy(&buffer).into_owned();
        if buffer == String::new() {
            Ok(None)
        } else {
            Ok(Some(buffer))
        }
    }
}
