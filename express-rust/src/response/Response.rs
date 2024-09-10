pub struct Response {
    protocol: String,
    status_code: String,
    headers: HashMap<String, String>,
    body: String
}

impl Response {
    fn new() -> Self {
        default_response()
    }

    fn status(&self, status_code) {
        self.status_code = status_code;
        self
    }
}

fn default_response() {
    Response {
        protocol: "HTTP/1.1",
        status_cdoe: "200",
        headers: (),
        body: "".to_string()
    }
}
