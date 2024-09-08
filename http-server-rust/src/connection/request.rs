use std::io::Read;
use std::net::TcpStream;

use super::header::Header;

#[derive(Debug)]
pub struct Request {
    pub http_protocol: String,
    method: String,
    path: String,
    headers: Vec<Header>,
    body: String,
}

impl Request {
    pub fn new(stream: &mut TcpStream) -> Self {
        let mut request_object: Request = Request {
            http_protocol: "".to_string(),
            method: "".to_string(),
            path: "".to_string(),
            headers: vec![],
            body: "".to_string(),
        };

        parse_stream(stream, &mut request_object);

        request_object
    }
}

fn parse_stream(stream: &mut TcpStream, request_object: &mut Request) {
    let mut buffer = [0; 512];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let request_data = String::from_utf8_lossy(&buffer[..bytes_read]);

            if let Some((request_header, request_body)) = request_data.split_once("\r\n\r\n") {
                request_object.body = request_body.to_string();
                parse_request_header(request_header, request_object);
            } else {
                eprintln!("Invalid Request");
            }
        }
        Err(error) => {
            eprintln!("Failed to read stream. Error: {error}");
        }
    }
}

fn parse_request_header(header: &str, request: &mut Request) {
    let header_array: Vec<&str> = header.split("\r\n").collect::<Vec<&str>>();
    let status_line = header_array[0].to_string();
    let status_line = status_line.split(' ').collect::<Vec<&str>>();

    request.method = status_line[0].to_string();
    request.path = status_line[1].to_string();
    request.http_protocol = status_line[2].to_string();

    request.headers = header_array[1..]
        .iter()
        .filter(|header| !header.is_empty() && header.contains(":"))
        .map(|header| {
            let header_array = header.split(" ").collect::<Vec<&str>>();

            Header {
                key: header_array[0]
                    .strip_suffix(":")
                    .unwrap_or(header_array[0])
                    .to_string(),
                value: header_array[1].to_string(),
            }
        })
        .collect::<Vec<Header>>();
}
