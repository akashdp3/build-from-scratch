mod app;
mod conn;

use app::App;

mod utils {
    pub mod server;
}

fn main() {
    let app = App::new();

    app.listen(5000);
}
