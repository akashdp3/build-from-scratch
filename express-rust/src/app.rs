use crate::utils::server::start_server;

pub struct App {}

impl App {
   pub fn new() -> Self {
       App {}
   }

   pub fn listen(&self, port: u16) {
       start_server(port);
   }
}
