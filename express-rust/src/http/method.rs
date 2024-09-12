pub struct Method(Inner);

enum Inner {
    Options,
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl Method {
    // OPTIONS
    pub const OPTIONS: Method = Method(Inner::Options);

    // GET
    pub const GET: Method = Method(Inner::Get);

    // POST
    pub const POST: Method = Method(Inner::Post);

    // PUT
    pub const PUT: Method = Method(Inner::Put);

    // PATCH
    pub const PATCH: Method = Method(Inner::Patch);

    // DELETE
    pub const DELETE: Method = Method(Inner::Delete);

    pub fn as_str(method_str: &str) -> Self {
        match method_str {
            "OPTIONS" => Method::OPTIONS,
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "PATCH" => Method::PATCH,
            "DELETE" => Method::DELETE,
            _ => panic!("Invalid Request"),
        }
    }
}
