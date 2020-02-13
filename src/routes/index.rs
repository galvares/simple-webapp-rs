use mustache;
use mustache::MapBuilder;

use rouille::{Request, Response};

use std::str;
use std::collections::HashMap;

use std::fs::{File};
use std::io::{BufReader, Read};

pub struct Index {}

impl Index {
    pub fn index() -> Result<String, String> {
        // template location
        let filename = format!("{}/index.html", ::WWW_PATH);
        
        // abrir arquivo de template e lê-lo
        let mut buffer: Vec<u8> = vec![];
        
        let fo = File::open(filename).expect("Couldn't open template file");
        let mut reader = BufReader::new(fo);
        reader
            .read_to_end(&mut buffer)
            .expect("Couldn't read the content of the template file");        
            

        // convert Vector u8 to str
        let buf = match str::from_utf8(&buffer[..]) {
            Ok(t) => t,
            Err(e) => panic!("Invalid char sequence: {}", e),
        };

        // renderizar com o mustache
        let mus = mustache::compile_str(buf)
            .expect("Failed to compile buffer");
        
        let data = MapBuilder::new()
            .build();

        // retorno
        match mus.render_data_to_string(&data) {
            Ok(t) => Ok(t),
            Err(e) => Err(format!("Couldn't render data to string")),
        }
    }
}
