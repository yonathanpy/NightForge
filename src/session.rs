use crate::fs::FileSystem;
use crate::logger::Logger;

pub struct Session<'a> {
    pub fs: FileSystem,
    pub logger: &'a Logger,
    pub username: Option<String>,
    pub authenticated: bool,
}

impl<'a> Session<'a> {
    pub fn new(logger: &'a Logger) -> Self {
        Session {
            fs: FileSystem::new(),
            logger,
            username: None,
            authenticated: false,
        }
    }

    pub fn handle_command(&mut self, cmd: &str) -> String {
        match cmd {
            "USER" => {
                self.username = Some("guest".to_string());
                self.logger.log("USER command received");
                "331 Username OK\n".to_string()
            }
            "PASS" => {
                self.authenticated = true;
                self.logger.log("PASS command received");
                "230 Login successful\n".to_string()
            }
            "LIST" => {
                let files = self.fs.list().join("\n");
                self.logger.log("LIST command executed");
                format!("{}\n", files)
            }
            _ => {
                self.logger.log(&format!("Unknown command: {}", cmd));
                "500 Unknown command\n".to_string()
            }
        }
    }
}
