use crate::bow::BOW;

use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

const DB_NAME: &str = "text_analyzer";
const COLL_NAME: &str = "bows";






pub async fn create() {
    let mut bow = BOW::default();
    let word = "word".to_string();

    bow.add_word(word.clone());


    
    // let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://root:example@localhost:27017/".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let collection = client.database(DB_NAME).collection(COLL_NAME);


    let result = collection.insert_one(bow).await;
    match result {
        Ok(_) => println!("user added"),
        Err(err) => println!("{}", err.to_string()),
    }
}
