#![allow(dead_code)]

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

const SERVER_IP: &str = "127.0.0.1:8080";

fn main() {
    // let string = String::from("sdfasdf");
    // let string_slice = &string[2..4];
    // let string_borrow = &string;
    // let string_literal = "Hello";

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let server = Server::new(SERVER_IP.to_string());
    server.run(WebsiteHandler::new(public_path));
}
