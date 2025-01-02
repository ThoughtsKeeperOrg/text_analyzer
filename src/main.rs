use text_analyzer::entities_updates_consumer;
use text_analyzer::mongo_database;
use text_analyzer::neo4j_database;

#[tokio::main]
async fn main() {
    mongo_database::prepare().await;
    neo4j_database::prepare().await;
    entities_updates_consumer::start().await;
}
