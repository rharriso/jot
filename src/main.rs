#[macro_use(doc, bson)]
extern crate bson;
#[macro_use]
extern crate serde_derive;

extern crate mongodb;
#[macro_use(load_yaml)]
extern crate clap;

mod note;

use mongodb::{ Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use clap::{App};
use note::Note;

static db_path: &'static str = "~/.jot.db";

// save a new note
fn saveNote (note: Note) {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize client");
    let collection = client.db("taker").collection("notes");
    collection.insert_one(note.to_doc(), None).expect("couldn't insert");
    println!("note saved: {:?}", note);
}

fn main () {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        println!("Printing lists");

        let tag_str = matches.value_of("tags").unwrap_or("no tags");
        println!("tags: {}", tag_str);

        if let Some(query_string) = matches.value_of("QUERY") {
            println!("query by: {}", query_string);
        }

    } else  {
        println!("adding a note");

        if let Some(file_input) = matches.value_of("file") {
            println!("File input: {}", file_input);
        }

        if let Some(note_content) = matches.value_of("NOTE_CONTENT") {
            saveNote(Note::new(note_content));
        }
    }

    println!("matches: {:?}", matches);
}

