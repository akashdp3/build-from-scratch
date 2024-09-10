use std::net::TcpListener;

pub fn start_server(server_address: String) {
    let listener = TcpListener::bind(server_address).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_) => {
                println!("Accpected new connection");
            },
            Err(error) => {
                eprintln!("Error on receiving request. Error: {error}");
            }
        }
    }
    
}

