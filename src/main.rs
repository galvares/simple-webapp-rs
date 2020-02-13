// simple web project using rouille and mustache.
// (c) galvares, 2018-02-18.

#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate postgres;

extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate mustache;

#[macro_use]
extern crate rouille;
use rouille::{Request, Response};

pub mod database;
pub mod server;
pub mod routes;

use database::Database;
use server::Server;

// www file
static WWW_PATH: &'static str = "/tmp/www";

// database settings
static DATABASE: &'static str = "postgres://postgres:123@localhost/doctorstrange";

// server settings
static SERVER_IP: &'static str = "0.0.0.0";
static SERVER_PORT: i32 = 8000;

fn main() {
    let db: Database = Database::new(DATABASE);

    match db.prepare() {
        Ok(t) => println!("[+] database seems ok. continue.."),
        Err(e) => panic!("[-] database seems failed to initiate. exiting!"),
    }

    let server: Server = Server::new(SERVER_IP, SERVER_PORT);

    println!("[+] listening on {}:{}", &server.ip, &server.port);    

    rouille::start_server(format!("{}:{}", server.ip, server.port), move |request| {
        server.run(&request, &db.conn)
    });    
}
