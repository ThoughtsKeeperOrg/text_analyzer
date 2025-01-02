use crate::bow::{compute_similarity, BOW};
use crate::bow_data_mapper;
use crate::neo4j_database::Client;
use futures::stream::StreamExt;

pub struct Processor {
    bow_mapper: Mapper,
    graph_client: Client,
}

impl Processor {
    pub async fn new() -> Self {
        Self {
            bow_mapper: bow_data_mapper::Mapper::new().await,
            graph_client: Client::new().await,
        }
    }

    pub async fn call(&self, text: String, entity_id: String) {
        let mut new_bow = BOW::from_text(text);
        new_bow.entity_id = entity_id;

        self.bow_mapper.save(&new_bow).await;

        let mut cursor = self.bow_mapper.all().await;
        while let Some(Ok(current_bow)) = cursor.next().await {
            self.update_similarity(&new_bow, &current_bow).await;
        }
        // in case if new graph nodes were created in other threads
        self.update_missing_similarities(&new_bow).await;
    }

    async fn update_similarity(&self, new_bow: &BOW, current_bow: &BOW) {
        if current_bow.entity_id == new_bow.entity_id {
            return;
        }
        let similarity = compute_similarity(&new_bow, &current_bow);

        self.graph_client
            .create_similarity_relation(&new_bow.entity_id, &current_bow.entity_id, similarity)
            .await;
    }

    async fn update_missing_similarities(&self, new_bow: &BOW) {
        let missing_ids = self
            .graph_client
            .find_missing_relations(&new_bow.entity_id)
            .await;
        for entity_id in missing_ids.iter() {
            if let Some(current_bow) = self.bow_mapper.find(entity_id).await {
                self.update_similarity(&new_bow, &current_bow).await;
            }
        }
    }
}

use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_process() {
    let graph_client = Client::new().await;
    let collection = bow_data_mapper::Mapper::new().await;
    collection.delete_all().await;

    let mut bow1 = BOW::default();
    bow1.entity_id = "111111".to_string();
    bow1.add_word("aaaaaa".to_string());
    bow1.add_word("bbbbbb".to_string());
    bow1.add_word("cccccx".to_string());
    bow1.add_word("xxxxxx".to_string());
    collection.save(&bow1).await;

    let mut bow2 = BOW::default();
    bow2.entity_id = "22222".to_string();
    bow2.add_word("aaaaaa".to_string());
    bow2.add_word("xxxxxx".to_string());
    collection.save(&bow2).await;

    let mut bow3 = BOW::default();
    bow3.entity_id = "333333".to_string();
    bow3.add_word("ooooooo".to_string());
    bow3.add_word("uuuuuuu".to_string());
    collection.save(&bow3).await;

    let text = "aaaaaa bbbbbb cccccc".to_string();
    let entity_id = "99999".to_string();

    let processor = Processor::new().await;

    processor.call(text, entity_id.clone()).await;

    let new_bow = collection.find(&entity_id).await.unwrap();
    assert_eq!(new_bow.words_count, 3);

    let estimation_from_db = graph_client
        .get_similarity_estimation(&entity_id, &bow1.entity_id)
        .await;
    assert_eq!(estimation_from_db, 0.725);

    let estimation_from_db = graph_client
        .get_similarity_estimation(&entity_id, &bow2.entity_id)
        .await;
    assert_eq!(estimation_from_db, 0.33333334);

    let estimation_from_db = graph_client
        .get_similarity_estimation(&entity_id, &bow3.entity_id)
        .await;
    assert_eq!(estimation_from_db, 0.0);

    collection.delete_all().await;
    let _ = graph_client.delete_all().await;
}
