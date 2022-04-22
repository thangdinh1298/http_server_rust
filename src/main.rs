extern crate ini;

mod http;
mod utils; //Do this so we can use crate:utils in server.rs
mod handlers;

use std::env;

use http::server;

use ini::Ini;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Ini::load_from_file(args.get(1).unwrap()).unwrap();
    let server = server::HTTPServer::new(conf);

    println!("Server starting");
    server.run();
}
