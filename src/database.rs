//extern crate postgres;
use postgres::{Connection, TlsMode};
//use self::postgres::transaction::Transaction;

use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(url: &str) -> Database {
        let conn = Connection::connect(url.to_string(), TlsMode::None);

        match conn {
            Ok(_) => Database { conn: Mutex::new(conn.unwrap()) },
            Err(_) => panic!("Can't create connection with database")
        }
    }

    pub fn prepare(&self) -> Result<bool, bool> {
        let prepare = "CREATE TABLE IF NOT EXISTS notes ( \
                       id SERIAL PRIMARY KEY, \
                       title TEXT NOT NULL,   \
                       content TEXT NOT NULL  \
                       );";

        match self.conn.lock().unwrap().execute(prepare, &[]) {
            Ok(t) => Ok(true),
            Err(e) => Err(false),
        }
    }    
}
