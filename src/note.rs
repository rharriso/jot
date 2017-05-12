extern crate uuid;

use self::uuid::Uuid;


pub struct Note {
    pub content: String,
    pub uuid: String,
}

impl Note {
    // create a new note from a content string
    pub fn new(content: &str) -> Note {
        return Note {
            content: content.to_string(),
            uuid: Uuid::new_v4().to_string()
        }
    }
}
