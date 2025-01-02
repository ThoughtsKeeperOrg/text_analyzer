use crate::bow_data_mapper;
use mongodb::{Client, Database};

pub async fn prepare() -> Database {
    let db = establish_connection().await;
    create_index(&db).await;
    db
}

pub async fn establish_connection() -> Database {
    let db_name = std::env::var("TEXT_ANALYZER_DB_NAME").unwrap_or_else(|_| "text_analyzer".into());

    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://root:example@localhost:27017/".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    client.database(&db_name)
}

async fn create_index(database: &Database) {
    bow_data_mapper::create_index(&database).await; // TODO: extract call to corresponding class
}

#[cfg(test)]
use tokio::time::timeout;
#[cfg(test)]
use tokio::time::Duration;

#[tokio::test]
async fn test_establish_connection() {
    establish_connection().await;
}

#[tokio::test]
async fn test_create_index() {
    std::env::set_var("TEXT_ANALYZER_DB_NAME", "test_text_analyzer");
    let db = establish_connection().await;

    if let Err(_) = timeout(Duration::from_millis(1000), create_index(&db)).await {
        panic!("DB timeout in test run");
    }

    let collection_names = db
        .list_collection_names()
        .await
        .expect("db queries should work");

    assert_eq!(collection_names[0], "bows");
}

