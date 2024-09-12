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
    pub const OPTIONS: Method = Method(Options);

    // GET
    pub const GET: Method = Method(Get);

    // POST
    pub const POST: Method = Method(Post);

    // PUT
    pub const PUT: Method = Method(Put);

    // PATCH
    pub const PATCH: Method = Method(Patch);

    // DELETE
    pub const DELETE: Method = Method(Delete);
}
