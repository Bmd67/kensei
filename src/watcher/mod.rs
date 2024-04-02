use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

pub struct FileWatcher {
    paths: Vec<String>,
    callback: Arc<Mutex<Box<dyn FnMut(Vec<String>) + Send>>>,
}

impl FileWatcher {
    pub fn new<F>(paths: Vec<String>, callback: F) -> Self
    where F: FnMut(Vec<String>) + Send + 'static
    {
        Self { paths, callback: Arc::new(Mutex::new(Box::new(callback))) }
    }

    pub fn start(&self) -> notify::Result<()> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        for path in &self.paths {
            watcher.watch(Path::new(path), RecursiveMode::Recursive)?;
            println!("Watching: {}", path);
        }
        while let Ok(Ok(event)) = rx.recv() {
            if let Some(p) = event.paths.first() {
                let mut cb = self.callback.lock().unwrap();
                cb(vec![p.to_string_lossy().to_string()]);
            }
        }
        Ok(())
    }
}
