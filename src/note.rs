extern crate uuid;
extern crate time;

use self::uuid::Uuid;
use self::time::Timespec;


pub struct Note {
    pub content: String,
    pub created_at: Timespec,
    pub uuid: String,
}

impl Note {
    // create a new note from a content string
    pub fn new(content: &str) -> Note {
        return Note {
            content: content.to_string(),
            created_at: time::get_time(),
            uuid: Uuid::new_v4().to_string()
        }
    }
}
