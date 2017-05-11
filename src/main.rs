#[macro_use(load_yaml)] extern crate clap;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod note;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use dotenv::dotenv;
use std::env;

use note::*;
use std::process::{Command, Stdio};
use clap::{App};

// save a new note
fn save_note (note: NewNote) {
    use schema::notes::dsl::*;

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let conn = SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    diesel::insert(&note)
        .into(schema::notes::table)
        .execute(&conn)
        .expect("Error saving note");

    // list notes
    let results = notes
        .limit(5)
        .load::<Note>(&conn)
        .expect("Error loading notes");

    for note in results {
        println!("{}", note.uuid);
        println!("{}", note.content);
        println!("\n");
    }

    /*
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize client");
    let collection = client.db("jot").collection("notes");
    collection.insert_one(note.to_doc(), None).expect("couldn't insert");
    println!("note saved: {:?}", note.to_doc());
    */
}


fn main () {
    dotenv().ok();

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

        // get input from file
        if let Some(file_input) = matches.value_of("file") {
            println!("File input: {}", file_input);
        }

        // get input from note content
        else if let Some(note_content) = matches.value_of("NOTE_CONTENT") {
            save_note(NewNote::new(note_content));

        // get input from editor
        } else if let Ok(editor) = std::env::var("EDITOR") {
            // editor writes to scratch
            let _ = Command::new(editor)
                .arg("/tmp/jot-scratch")
                .spawn()
                .expect("failed to get $EDITOR output")
                .wait();

            // read and delete scratch space scratch
            let read_command = Command::new("sh")
                .arg("-c")
                .arg("cat /tmp/jot-scratch && rm /tmp/jot-scratch")
                .arg("/tmp/jot-scratch")
                .stdout(Stdio::piped())
                .spawn()
                .expect("couldn't cat out the scratch");
            let output = read_command.wait_with_output().expect("command didn't exit");

            let note_content = String::from_utf8(output.stdout)
                .expect("bad $EDITOR result.");
            save_note(NewNote::new(&note_content.trim()));
        } else {
            panic!("Not input method given, failing");
        }
    }
}

