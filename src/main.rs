use text_analyzer::bow_data_mapper;
use text_analyzer::entities_updates_consumer;
use text_analyzer::neo4j_repository;

#[tokio::main]
async fn main() {
    bow_data_mapper::prepare().await;
    neo4j_repository::prepare().await;
    entities_updates_consumer::start().await;
}
