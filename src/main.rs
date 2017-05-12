#[macro_use(load_yaml)] extern crate clap;
extern crate rusqlite;

mod note;

use std::env;
use std::path::Path;
use std::fs;

use note::Note;
use std::process::{Command, Stdio};
use clap::{App};
use rusqlite::Connection;

// save a new note
fn save_note (note: Note) {
    let mut db_file = env::home_dir().expect("couln't get home dir");
    db_file.push(".jot");
    db_file.push("db.sql");
    let conn = Connection::open(db_file).expect("unable to create db.");

    conn.execute("INSERT INTO notes (content, uuid)
                  VALUES (?1, ?2)",
                 &[&note.content, &note.uuid]).expect("Couldn't insert note.");
}

fn init () {
    let mut jot_dir = env::home_dir().expect("couln't get home dir");
    jot_dir.push(".jot");

    // create the dir
    fs::create_dir(jot_dir.clone());

    let mut db_file = jot_dir;
    db_file.push("db.sql");
    let conn = Connection::open(db_file).expect("unable to create db.");

    conn.execute("CREATE TABLE notes (
        id      INTEGER PRIMARY KEY NOT NULL,
        content TEXT NOT NULL,
        uuid    TEXT NOT NULL
    )", &[]).expect("couldn't create table");
}


fn main () {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("init") {
        println!("Creating table");
        init();

    } else if let Some(matches) = matches.subcommand_matches("list") {
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
            save_note(Note::new(note_content));

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
            save_note(Note::new(&note_content.trim()));
        } else {
            panic!("Not input method given, failing");
        }
    }
}

