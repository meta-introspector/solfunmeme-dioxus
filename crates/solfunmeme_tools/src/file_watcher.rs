use notify::{Watcher, RecursiveMode, Result};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch<P: AsRef<Path>>(path: P) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        match res {
            Ok(event) => tx.send(event).unwrap(),
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    println!("Watching for changes...");

    loop {
        match rx.recv() {
            Ok(event) => {
                println!("Change detected: {:?}", event);
                // Here you can trigger the re-processing logic
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
