use std::io::{Error, Read, Write};
use std::net::TcpStream;

use crate::utils::server::start_server;

pub struct App {}

impl App {
    pub fn new() -> Self {
        App {}
    }

    pub fn listen(&self, port: u16) {
        start_server(port, |mut stream| self.handle_request(&mut stream));
    }

    fn handle_request(&self, stream: &mut TcpStream) -> Result<(), Error> {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let request_string = String::from_utf8_lossy(&buffer[..bytes_read]);

                let response = "HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello";
                stream.write(response.as_bytes()).expect("Error");
            }
            Err(error) => {
                println!("Error occurred  while reading stream. Error: {error}");
                return Err(error);
            }
        }

        Ok(())
    }
}
