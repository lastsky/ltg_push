use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc;
use std::time::Duration;
use std::path::PathBuf;

mod diff;
use self::diff::DiffFinder;
use error::Error;
use config::FileConfig;
use telegram::Telegram;


pub struct LogWatcher {
    files: Vec<FileConfig>,
    notifyer: RecommendedWatcher,
    receiver: mpsc::Receiver<DebouncedEvent>,
    diff_finder: DiffFinder,
    telegram: Telegram,
}
impl LogWatcher {
    pub fn new(files: Vec<FileConfig>, tg: Telegram) -> Result<LogWatcher, Error> {
        let (tx, rx) = mpsc::channel();
        let watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;

        Ok(LogWatcher {
            files: files.to_vec(),
            notifyer: watcher,
            receiver: rx,
            diff_finder: DiffFinder::new(files)?,
            telegram: tg,
        })
    }
    pub fn watch(&mut self) -> Result<(), Error> {
        for file in &self.files {
            self.notifyer.watch(&file.path, RecursiveMode::NonRecursive)?;
        }

        loop {
            match self.receiver.recv() {
                Ok(DebouncedEvent::Write(path)) => self.notify(path)?,
                Ok(_) => {}
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
    fn notify(&mut self, path: PathBuf) -> Result<(), Error> {
        let path = match path.to_str() {
            Some(path) => path,
            None => return Err(Error::Text(format!("could not find path"))),
        };
        let diff = self.diff_finder.find(path)?;
        match diff {
            Some(diff) => self.telegram.send(path, diff)?,
            None => {}
        };

        Ok(())
    }
}
