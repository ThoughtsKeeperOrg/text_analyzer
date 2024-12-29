use text_analyzer::entities_updates_consumer;
use text_analyzer::mongo_database;
use text_analyzer::neo4j_database;

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| {
        mongo_database::init();
    })
    .await
    .expect("Mongo initialization failed");
    neo4j_database::Client::init()
        .await
        .create_constraints()
        .await;

    entities_updates_consumer::start().await;
}
