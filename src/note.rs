extern crate chrono;
extern crate uuid;

use self::chrono::prelude::*;
use self::uuid::Uuid;

#[derive(Debug)]
pub struct Note {
    content: String,
    created_at: DateTime<UTC>,
    uuid: String,
}

impl Note {
    // create a new note from a content string
    pub fn new(content: &str) -> Note {
        return Note{
            content: content.to_string(),
            created_at: UTC::now(),
            uuid: Uuid::new_v4().to_string()
        }
    }
}