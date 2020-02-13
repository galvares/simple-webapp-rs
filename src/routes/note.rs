use mustache;
use mustache::MapBuilder;

use rouille;
use rouille::{Request, Response};

use std::str;
use std::collections::HashMap;

use std::fs::{File};
use std::io::{BufReader, Read};

use postgres::transaction::Transaction;

pub struct Note {
    id: i32,
    title: String,
    content: String,
}

impl Note {
    fn new(id: i32, title: String, content: String) -> Note {
        Note {
            id: id,
            title: title,
            content: content,
        }
    }
    
    pub fn read(id: i32, database: &Transaction) -> Result<String, String> {
        let filename = format!("{}/note_read.html", ::WWW_PATH);
        
        let mut cid: Option<i32> = None;
        let mut note: Note = Note::new(0, String::new(), String::new());

        for row in &database.query("SELECT id, title, content FROM notes WHERE id = $1", &[&id]).unwrap() {
            cid = Some(row.get(0));
            note = Note::new(row.get(0), row.get(1), row.get(2));
        }

        match cid {
            Some(c) => {
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
                    .insert_str("id", note.id.to_string())
                    .insert_str("title", note.title)
                    .insert_str("content", note.content)
                    .build();

                // retornar
                match mus.render_data_to_string(&data) {
                    Ok(t) => Ok(t),
                    Err(e) => Err(format!("Couldn't render data to string")),
                }
            },
            
            None => Err(format!("None note exists with ID #{}", id)),
        }
    }

    pub fn update(request: &Request, id: i32, database: &Transaction) -> Result<String, String> {
        let body = match post_input!(request, { title: String, content: String })  {
            Ok(t) => Ok(t),
            Err(e) => {
                let json = rouille::try_or_400::ErrJson::from_err(&e);
                Err(Response::json(&json).with_status_code(400))
            }
        };
        
        let body = body.unwrap();

        match database.execute("UPDATE notes SET title = $1 , content = $2 WHERE id = $3",
                               &[&body.title, &body.content, &id]) {
            Ok(t) => Ok(format!("The note was updated with success!")), 
            Err(e) => Err(format!("The note hasn't been updated. Try again!")),
        }
    }
    
    pub fn add(request: &Request, database: &Transaction) -> Result<String, String> {
        let body = match post_input!(request, { title: String, content: String })  {
            Ok(t) => Ok(t),
            Err(e) => {
                let json = rouille::try_or_400::ErrJson::from_err(&e);
                Err(Response::json(&json).with_status_code(400))
            }
        };

        let body = body.unwrap();

        let mut id: Option<i32> = None;

        for row in &database.query("INSERT INTO notes (title, content) VALUES ($1, $2) RETURNING id",
                                   &[&body.title, &body.content]).unwrap() {
            id = Some(row.get(0));
        }

        match id {
            Some(t) => Ok(format!("/note/{}#added", id.unwrap())),
            None => Err(format!("Some error occured and INSERT didn't work properly")),            
        }
    }

    pub fn delete(id: i32, database: &Transaction) -> Result<String, String> {
        match database.execute("DELETE FROM notes WHERE id = $1", &[&id]) {
            Ok(t) => Ok(format!("The note was deleted with success!")), 
            Err(e) => Err(format!("Some error occured and DELETE didn't work properly")),
        }
    }

    pub fn list(database: &Transaction) -> Result<String, String> {
        let filename = format!("{}/note_list.html", ::WWW_PATH);
        
        let mut notes: Vec<Note> = vec![];

        for row in &database.query("SELECT id, title, content FROM notes", &[]).unwrap() {
            notes.push(Note::new(row.get(0), row.get(1), row.get(2)));
        }

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
            .insert_vec("notes", |mut mapper| {
                for n in notes.iter() {
                    mapper = mapper.push_map(|mapper| {
                        mapper
                            .insert_str("id", n.id.to_string())
                            .insert_str("title", n.title.to_string())
                    });
                }
                mapper
            })
            .build();

        // retornar
        match mus.render_data_to_string(&data) {
            Ok(t) => Ok(t),
            Err(e) => Err(format!("Couldn't render data to string")),
        }
    }    
}
