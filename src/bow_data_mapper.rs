use crate::bow::BOW;
use crate::mongo_database;
use mongodb::{bson, Collection, Cursor};
use mongodb::{bson::doc, options::IndexOptions, Database, IndexModel};

const COLLECTION_NAME: &str = "bows";

pub struct Mapper {
    pub db: Database,
    pub collection: Collection<BOW>,
}

impl Mapper {
    pub async fn new() -> Self {
        let db = mongo_database::establish_connection().await;
        let collection = db.collection::<BOW>(COLLECTION_NAME);
        Self {
            db: db,
            collection: collection,
        }
    }

    pub async fn save(&self, entity: &BOW) {
        let filter = doc! { "entity_id": entity.entity_id.clone() };
        let data = doc! { "$set": bson::to_bson(&entity).expect("Entity should be serializable") };
        self.collection
            .update_one(filter, data)
            .upsert(true)
            .await
            .unwrap();
    }

    pub async fn find(&self, entity_id: &String) -> Option<BOW> {
        let filter = doc! { "entity_id": entity_id.clone() };
        self.collection.find_one(filter).await.unwrap()
    }

    pub async fn count(&self) -> u64 {
        self.collection.count_documents(doc! {}).await.unwrap()
    }

    pub async fn delete_all(&self) {
        self.collection.delete_many(doc! {}).await.unwrap();
    }

    pub async fn all(&self) -> Cursor<BOW> {
        self.collection.find(doc! {}).await.unwrap()
    }
}

pub async fn create_index(database: &Database) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "entity_id": 1 })
        .options(options)
        .build();
    database
        .collection::<BOW>(COLLECTION_NAME)
        .create_index(model)
        .await
        .expect("creating an index should succeed");
}

#[cfg(test)]
use crate::mongo_database::establish_connection;
#[cfg(test)]
use futures::stream::StreamExt;
#[cfg(test)]
use tokio::time::timeout;
#[cfg(test)]
use tokio::time::Duration;

#[tokio::test]
async fn test_crud() {
    std::env::set_var("TEXT_ANALYZER_DB_NAME", "test_text_analyzer");
    mongo_database::prepare().await;

    let collection = Mapper::new().await;

    collection.delete_all().await;

    let mut bow = BOW::default();
    bow.entity_id = "11".to_string();
    bow.add_word("word".to_string());

    collection.save(&bow).await;
    collection.save(&bow).await;
    bow.add_word("other".to_string());
    collection.save(&bow).await;

    assert_eq!(collection.count().await, 1);

    let bow_from_db = collection.find(&bow.entity_id).await.unwrap();
    assert_eq!(bow, bow_from_db);

    let mut bow2 = BOW::default();
    bow2.entity_id = "2".to_string();
    bow2.add_word("word".to_string());
    bow2.add_word("word".to_string());

    collection.save(&bow2).await;
    assert_eq!(collection.count().await, 2);

    let items = vec![bow, bow2];

    let mut i: usize = 0;
    let mut cursor = collection.all().await;

    while let Some(doc) = cursor.next().await {
        assert_eq!(doc.unwrap(), items[i]);
        i += 1;
    }

    collection.delete_all().await;
    assert_eq!(collection.count().await, 0);
}

#[tokio::test]
async fn test_create_index() {
    std::env::set_var("TEXT_ANALYZER_DB_NAME", "test_text_analyzer");
    let db = establish_connection().await;

    if let Err(_) = timeout(Duration::from_millis(1000), create_index(&db)).await {
        panic!("DB timeout test run");
    }

    let collection_names = db
        .list_collection_names()
        .await
        .expect("db queries should work");

    assert_eq!(collection_names[0], "bows");
}
