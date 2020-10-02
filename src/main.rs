#![allow(dead_code)]

use server::Server;

mod http;
mod server;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(website_handler::WebsiteHandler);
}
