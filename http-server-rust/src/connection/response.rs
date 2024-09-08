use super::header::Header;
use super::request::Request;

pub struct Response {
    http_protocol: String,
    headers: Vec<Header>,
    status_code: String,
    body: String,
}

impl Response {
    fn new(request: Request) -> Self {
        Response {
            http_protocol: request.http_protocol,
            headers: vec![],
            status_code: "200".to_string(),
            body: "".to_string(),
        }
    }
}
