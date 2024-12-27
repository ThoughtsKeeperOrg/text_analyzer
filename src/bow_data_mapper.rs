use crate::bow::BOW;
use crate::mongo_database;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::{bson, Cursor};
use mongodb::{bson::doc, options::IndexOptions, Database, IndexModel};

const COLLECTION_NAME: &str = "bows";

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

pub async fn save(entity: &BOW) {
    let db = mongo_database::establish_connection().await;
    let collection = db.collection::<BOW>(COLLECTION_NAME);
    let filter = doc! { "entity_id": entity.entity_id.clone() };
    let data = doc! { "$set": bson::to_bson(&entity).expect("Entity should be serializable") };
    collection
        .update_one(filter, data)
        .upsert(true)
        .await
        .unwrap();
}

pub async fn find(entity_id: &String) -> Option<BOW> {
    let db = mongo_database::establish_connection().await;
    let collection = db.collection::<BOW>(COLLECTION_NAME);
    let filter = doc! { "entity_id": entity_id.clone() };
    collection.find_one(filter).await.unwrap()
}

pub async fn count() -> u64 {
    let db = mongo_database::establish_connection().await;
    let collection = db.collection::<BOW>(COLLECTION_NAME);
    collection.count_documents(doc! {}).await.unwrap()
}

pub async fn delete_all() {
    let db = mongo_database::establish_connection().await;
    let collection = db.collection::<BOW>(COLLECTION_NAME);
    collection.delete_many(doc! {}).await.unwrap();
}

pub async fn all() -> Cursor<BOW> {
    let db = mongo_database::establish_connection().await;
    let collection = db.collection::<BOW>(COLLECTION_NAME);
    collection.find(doc! {}).await.unwrap()
}

#[cfg(test)]
use crate::mongo_database::establish_connection;
#[cfg(test)]
use tokio::time::timeout;
#[cfg(test)]
use tokio::time::Duration;

#[tokio::test]
async fn test_crud() {
    tokio::task::spawn_blocking(|| {
        std::env::set_var("TEXT_ANALYZER_DB_NAME", "test_text_analyzer");
        mongo_database::init();
    })
    .await
    .expect("Task panicked");
    delete_all().await;

    let mut bow = BOW::default();
    bow.entity_id = "11".to_string();
    bow.add_word("word".to_string());

    save(&bow).await;
    save(&bow).await;
    bow.add_word("other".to_string());
    save(&bow).await;

    assert_eq!(count().await, 1);

    let bow_from_db = find(&bow.entity_id).await.unwrap();
    assert_eq!(bow, bow_from_db);

    let mut bow2 = BOW::default();
    bow2.entity_id = "2".to_string();
    bow2.add_word("word".to_string());
    bow2.add_word("word".to_string());

    save(&bow2).await;
    assert_eq!(count().await, 2);

    let items = vec![bow, bow2];

    let mut i: usize = 0;
    let mut cursor = all().await;

    while let Some(doc) = cursor.next().await {
        assert_eq!(doc.unwrap(), items[i]);
        i += 1;
    }

    delete_all().await;
    assert_eq!(count().await, 0);
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

// dotenv().ok();
// let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

// if !Sqlite::database_exists(&database_url)
//     .await
//     .unwrap_or(false)
// {
//     let database_folder = env::var("DATABASE_FOLDER").expect("DATABASE_FOLDER must be set");
//     match fs::create_dir_all(database_folder) {
//         Err(error) => panic!("Database folder cannot be created: {:?}", error),
//         Ok(_) => {
//             debug_println!("Database folder is created")
//         }
//     }

//     debug_println!("Creating database {}", &database_url);
//     match Sqlite::create_database(&database_url).await {
//         Ok(_) => {
//             debug_println!("Create db success")
//         }
//         Err(error) => panic!("error: {}", error),
//     }
// } else {
//     debug_println!("Database already exists");
// }

// let db = SqlitePool::connect(&database_url).await.unwrap();

// let migrations_dir = std::env::var("MIGRATIONS_DIR").unwrap();
// let migrations = std::path::Path::new(&migrations_dir);
// let migration_results = sqlx::migrate::Migrator::new(migrations)
//     .await
//     .unwrap()
//     .run(&db)
//     .await;
// match migration_results {
//     Ok(_) => {
//         debug_println!("Migration success")
//     }
//     Err(error) => {
//         panic!("error: {}", error);
//     }
// }
// debug_println!("migration: {:?}", migration_results);

// db
// }

// // const DB_NAME: &str = "text_analyzer";

// pub async fn create() {
//     let mut bow = BOW::default();
//     let word = "word".to_string();

//     bow.add_word(word.clone());

//     // let db_name = std::env::var("TEXT_ANALYZER_DB_NAME")
//     //     .unwrap_or_else(|_| "text_analyzer".into());

//     // let uri = std::env::var("MONGODB_URI")
//     //     .unwrap_or_else(|_| "mongodb://root:example@localhost:27017/".into());

//     // let client = Client::with_uri_str(uri).await.expect("failed to connect");
//     // let collection = client.database(db_name);//.collection(COLL_NAME);

//     let result = collection.insert_one(bow).await;
//     match result {
//         Ok(_) => println!("user added"),
//         Err(err) => println!("{}", err.to_string()),
//     }
// }
