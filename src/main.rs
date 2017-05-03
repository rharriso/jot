//#[macro_use(bson, doc)]
//extern crate bson;
//extern crate mongodb;
#[macro_use(load_yaml)]
extern crate clap;

//use bson::Bson;
//use mongodb::{ Client, ThreadedClient};
//use mongodb::db::ThreadedDatabase;
use clap::{App};


fn main () {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("list") {
        println!("Printing lists");

        let tag_str = matches.value_of("tags").unwrap_or("no tags");
        println!("tags: {}", tag_str);
    } else {
        println!("adding a note");

        if let Some(fileInput) = matches.value_of("file") {
            println!("File input: {}", fileInput);
        }
    }

    println!("matches: {:?}", matches);
}

/*fn main() {
    //let client = Client::with_uri("mongodb://rharriso:Q55YMIZEj08a@ds129641.mlab.com:29641/")
    //    .expect("Failed to initialize");
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize client");

    let collection = client.db("taker").collection("notes");

    collection.insert_one(doc!{ "title" => "My guy is my guy ok guy?"}, None)
        .ok().expect("Failed to insert document.");

    let mut cursor = collection.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            if let Some(&Bson::String(ref title)) = item.get("title") {
                println!("title: {}", title);
            }
        }
    }
}
*/
