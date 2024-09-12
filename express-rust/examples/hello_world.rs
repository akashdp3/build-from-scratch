use express_rust::App;

/*
    const express = require('express')
    const app = express()

    app.get('/', function (req, res) {
    res.send('Hello World')
    })

    app.listen(3000)
*/
fn main() {
    let express = App::new();

    // app.get("/", fn (req, res) {
    //     res.send('Hello World');
    // });

    express.listen(5000);
}
