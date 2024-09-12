use core::panic;
use std::{
    io::Error,
    net::{TcpListener, TcpStream},
};

pub fn start_server<F>(port: u16, mut handler: F)
where
    F: FnMut(&mut TcpStream) -> Result<(), Error>,
{
    let listener = create_listener(port);

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Err(error) = handler(&mut stream) {
                    eprintln!("Error handling connection: {}", error);
                }
            }
            Err(error) => {
                eprintln!("Error on receiving request. Error: {error}");
            }
        }
    }
}

fn create_listener(port: u16) -> TcpListener {
    let server_address = format!("localhost:{port}");

    match TcpListener::bind(server_address) {
        Ok(listener) => {
            println!("Listening on port {port}");
            listener
        }
        Err(error) => {
            eprintln!("Error occured while trying to listen on {port}. Error: {error}");
            panic!("Could not start server on {port}");
        }
    }
}
