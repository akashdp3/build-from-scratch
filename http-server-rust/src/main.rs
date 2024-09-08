use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

mod connection;
use connection::{Request, Response};

const SERVER_ADDR: &str = "127.0.0.1";
const PORT: &str = "4221";

fn main() {
    let listener = TcpListener::bind(format!("{SERVER_ADDR}:{PORT}")).unwrap();
    println!("Ready to listen to incoming requests.");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Accepted new connection");

                handle_connection(&mut stream);

                stream.write("HTTP/1.1 200 OK\r\n".as_bytes()).unwrap();
            }
            Err(error) => {
                eprintln!("Failed to listen to incoming stream. Error: {error}");
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let request = Request::new(stream);
}
