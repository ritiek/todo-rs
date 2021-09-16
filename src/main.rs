use chrono::{DateTime, Local};
use mongodb::{
    bson::doc,
    results::InsertOneResult,
    sync::{Client, Collection},
};
use serde::{Deserialize, Serialize};
// use std::io::{self, BufRead};

const DB_NAME: &str = "todo-rs";
const COLLECTION_NAME: &str = "notes";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_on: DateTime<Local>,
}

impl Note {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            description: "".to_string(),
            completed: false,
            created_on: Local::now(),
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn create(self) -> Result<Self, ()> {
        Ok(Self {
            title: self.title,
            description: self.description,
            completed: self.completed,
            created_on: self.created_on,
        })
    }

    pub fn summarize(&self) -> String {
        let created_on_fmt = self.created_on.format("%d/%m/%Y @ %I:%M:%S %p");
        format!(
            "{}\n{}\n(Created on {})",
            self.title, self.description, created_on_fmt
        )
    }

    pub fn save_to(
        &self,
        collection: &mut Collection<Self>,
    ) -> mongodb::error::Result<InsertOneResult> {
        Ok(collection.insert_one(self, None)?)
    }
}

fn main() -> mongodb::error::Result<()> {
    let db_connection_uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(db_connection_uri)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Connected successfully.\n");

    let mut collection = client.database(DB_NAME).collection::<Note>(COLLECTION_NAME);

    // let mut test_note = Note::from_title("Test Note".to_string());
    // test_note.with_description("this is a sample test note".to_string());
    let test_note = Note::new()
        .with_title("Test Note".to_string())
        .with_description("this is a sample test note".to_string())
        .create()
        .unwrap();

    // println!("{}\n", test_note.summarize());
    // let result = test_note.save_to(&mut collection)?;
    // println!("{:?}\n", result);

    // println!("Todo app");
    // let choices = vec!["read"];
    // let stdin = io::stdin();
    // for line in stdin.lock().lines() {
    //     println!("{}", line.unwrap());
    // }

    // let note_from_rust_driver = collection.find_one(doc! { "title": "Note from Rust driver" }, None)?;
    // let note_from_rust_driver = collection.find(doc! { "title": "Note from Rust driver" }, None)?;
    // let note_from_rust_driver = collection.find(None, None)?;

    let saved_notes = collection.find(None, None)?;
    // println!("{:#?}", saved_notes);
    println!("Summary all saved notes:\n");
    for saved_note in saved_notes {
        let doc = saved_note?;
        println!("{}\n", doc.summarize());
    }

    // collection.insert_many(docs, None);

    // for collection_name in db.list_collection_names(None)? {
    //     println!("{}", collection_name);
    // }
    Ok(())
}
