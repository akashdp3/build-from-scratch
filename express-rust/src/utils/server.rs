use core::panic;
use std::net::TcpListener;

pub fn start_server(port: u16) {
    let listener = create_listener(port);

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("Accpected new connection");
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
