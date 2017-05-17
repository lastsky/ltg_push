use notify;
use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};
use std::sync::mpsc;
use std::time::Duration;

use config::FileConfig;


pub struct LogWatcher {
    files: Vec<FileConfig>,
    notifyer: RecommendedWatcher,
    receiver: mpsc::Receiver<DebouncedEvent>,
}
impl LogWatcher {
    pub fn new(files: Vec<FileConfig>) -> notify::Result<LogWatcher> {
        let (tx, rx) = mpsc::channel();
        let watcher: RecommendedWatcher = try!(Watcher::new(tx, Duration::from_secs(1)));

        Ok(LogWatcher {
            files: files,
            notifyer: watcher,
            receiver: rx,
        })
    }
    pub fn watch(&mut self) -> notify::Result<()> {
        for file in &self.files {
            self.notifyer.watch(&file.path, RecursiveMode::NonRecursive)?;
        }

        loop {
            match self.receiver.recv() {
                Ok(DebouncedEvent::Write(ref path)) => println!("{:?}", path),
                Ok(_) => {}
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}
