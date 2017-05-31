use notify::{RecommendedWatcher, Watcher, RecursiveMode, RawEvent, op};
use std::sync::mpsc;
use std::path::PathBuf;

mod diff;
mod matcher;
use self::diff::DiffFinder;
use self::matcher::Matcher;
use error::Error;
use config::FileConfig;
use telegram::Telegram;


pub struct LogWatcher {
    files: Vec<FileConfig>,
    notifyer: RecommendedWatcher,
    receiver: mpsc::Receiver<RawEvent>,
    diff_finder: DiffFinder,
    telegram: Telegram,
    matcher: Matcher,
}
impl LogWatcher {
    pub fn new(files: Vec<FileConfig>, tg: Telegram) -> Result<LogWatcher, Error> {
        let (tx, rx) = mpsc::channel();
        let watcher: RecommendedWatcher = Watcher::new_raw(tx)?;

        Ok(LogWatcher {
            files: files.to_vec(),
            notifyer: watcher,
            receiver: rx,
            diff_finder: DiffFinder::new(files.to_vec())?,
            telegram: tg,
            matcher: Matcher::new(files)?,
        })
    }
    pub fn watch(&mut self) -> Result<(), Error> {
        for file in &self.files {
            self.notifyer.watch(&file.path, RecursiveMode::Recursive)?;
        }

        loop {
            match self.receiver.recv() {
                Ok(RawEvent { path: Some(path), op: Ok(opb), cookie: _ }) => {
                    match opb {
                        op::WRITE => self.notify(path)?,
                        op::REMOVE => self.notify_remove(path)?,
                        _ => {}
                    }
                }
                Ok(_) => {}
                Err(e) => return Err(Error::Recv(e)),
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
            Some(diff) => {
                if self.matcher.is_matches(path, diff.clone()) {
                    let message = format!("*{}*\n```\n{}```", path, diff);
                    self.telegram.send(message)?
                }
            }
            None => {}
        };

        Ok(())
    }
    fn notify_remove(&mut self, path: PathBuf) -> Result<(), Error> {
        let path = match path.to_str() {
            Some(path) => path,
            None => return Err(Error::Text(format!("could not find path"))),
        };

        let message = format!("Watching file *{}* stopped (file removed)", path);
        self.telegram.send(message)?;

        Ok(())
    }
}
