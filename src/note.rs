extern crate uuid;
extern crate chrono;
extern crate bson;

use self::chrono::prelude::*;
use self::uuid::Uuid;
use self::bson::Document;

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub content: String,
    //pub created_at: DateTime<UTC>,
    #[serde(rename = "_id")]
    pub uuid: String,
}

impl Note {
    // create a new note from a content string
    pub fn new(content: &str) -> Note {
        return Note{
            content: content.to_string(),
//            created_at: UTC::now(),
            uuid: Uuid::new_v4().to_string()
        }
    }

    pub fn to_doc(&self) -> bson::Document {
        let mut doc = Document::new();
        doc.insert("content", self.content.to_string());
        doc.insert("uuid", self.uuid.to_string());
 //     doc.insert("created_at", self.created_at);
        return doc;
    }
}