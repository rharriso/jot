extern crate uuid;

use self::uuid::Uuid;
use chrono::prelude::*;


pub struct Note {
    pub content: String,
    pub created_at: NaiveDateTime,
    pub uuid: String,
}

impl Note {
    // create a new note from a content string
    pub fn new(content: &str) -> Note {
        return Note {
            content: content.to_string(),
            created_at: UTC::now().naive_utc(),
            uuid: Uuid::new_v4().to_string()
        }
    }
}
