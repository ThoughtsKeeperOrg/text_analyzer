use mongodb::{Client, Database};

pub async fn establish_connection() -> Database {
    let db_name = std::env::var("TEXT_ANALYZER_DB_NAME").unwrap_or_else(|_| "text_analyzer".into());

    let uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://root:example@localhost:27017/".into());

    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    client.database(&db_name)
}

#[tokio::test]
async fn test_establish_connection() {
    establish_connection().await;
}
