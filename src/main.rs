mod logger;
mod parser;
mod session;
mod fs;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use logger::Logger;
use session::Session;

fn handle_client(mut stream: TcpStream, logger: Logger) {
    let mut buffer = [0; 1024];
    let mut session = Session::new(&logger);

    loop {
        let size = match stream.read(&mut buffer) {
            Ok(0) => break, // client disconnected
            Ok(n) => n,
            Err(_) => break,
        };

        let input = String::from_utf8_lossy(&buffer[..size]).to_string();
        let command = parser::parse(&input);
        let response = session.handle_command(command);

        if stream.write(response.as_bytes()).is_err() {
            break;
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:2121").expect("Could not bind port");
    let logger = Logger::new();

    println!("NightForge listening on 0.0.0.0:2121");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let log_clone = Logger::new();
                thread::spawn(move || {
                    handle_client(stream, log_clone);
                });
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}
