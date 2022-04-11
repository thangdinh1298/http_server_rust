mod http;
mod utils; //Do this so we can use crate:utils in server.rs
mod handlers;

use http::server;

fn main() {
    println!("Hello, world!");
    let server = server::HTTPServer::new(4, 20);
    server.run();
}
