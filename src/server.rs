// web framework
use rouille;
use rouille::{Request, Response};

// database
use database::Database;
use postgres::Connection;
use postgres::transaction::Transaction;

// routes functions
use routes::index::Index;
use routes::note::Note;

use std::io;
use std::sync::Mutex;

pub struct Server {
    pub ip: String,
    pub port: i32,
}

impl Server {
    pub fn new(ip: &str, port: i32) -> Server {
        Server {
            ip: ip.to_string(),
            port: port,
        }
    }

    pub fn run(&self, request: &Request, conn: &Mutex<Connection>) -> Response {
        let db = conn.lock().unwrap();
        let db = match db.transaction() {
            Ok(t) => t,
            Err(e) => panic!("Some error occured in transaction DB"),
        };

        let response = self.routes(&request, &db);

        if response.is_success() {
            db.commit().unwrap();
        }

        response        
    }
   
    pub fn routes(&self, request: &Request, database: &Transaction) -> Response {
        router!(request,
                
                // Index routes
                (GET)(/) => {
                    match Index::index() {
                        Ok(t)  => Response::html(t),
                        Err(e) => Response::text(e).with_status_code(400),
                    }
                },

                // Note routes
                (GET)(/notes) => {
                    match Note::list(&database) {
                        Ok(t)  => Response::html(t),
                        Err(e) => Response::text(e).with_status_code(400),
                    }
                },

                (POST)(/note) => {
                    match Note::add(&request, &database) {
                        Ok(t)  => Response::redirect_303(t),
                        Err(e) => Response::text(e).with_status_code(400),
                    }
                },                

                (GET)(/note/{id: i32}) => {
                    match Note::read(id, &database) {
                        Ok(t)  => Response::html(t),
                        Err(e) => Response::text(e).with_status_code(400),
                    }
                },

                (PUT)(/note/{id: i32}) => {
                    match Note::update(&request, id, &database) {
                        Ok(t)  => Response::text(t).with_status_code(200), 
                        Err(e) => Response::text(e).with_status_code(400),
                    }
                },

                (DELETE)(/note/{id: i32}) => {
                    match Note::delete(id, &database) {
                        Ok(t)  => Response::text(t).with_status_code(200),
                        Err(e) => Response::text(e).with_status_code(400),
                    }
                },

                // Undefined / Invalid request
                _ => Response::redirect_303("/")
        )
    }
}
