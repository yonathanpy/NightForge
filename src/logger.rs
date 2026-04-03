use std::sync::{Arc, Mutex};

pub struct Logger {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn log(&self, entry: &str) {
        let mut guard = self.logs.lock().unwrap();
        guard.push(entry.to_string());
        println!("{}", entry); // real-time console output
    }

    pub fn all(&self) -> Vec<String> {
        let guard = self.logs.lock().unwrap();
        guard.clone()
    }
}
