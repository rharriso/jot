extern crate uuid;

use self::uuid::Uuid;
use super::schema::notes;


#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub content: String,
    pub uuid: String,
}

#[derive(Insertable)]
#[table_name="notes"]
pub struct NewNote {
    pub content: String,
    pub uuid: String,
}


impl NewNote {
    // create a new note from a content string
    pub fn new(content: &str) -> NewNote {
        return NewNote {
            content: content.to_string(),
            uuid: Uuid::new_v4().to_string()
        }
    }
}
