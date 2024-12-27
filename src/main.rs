// use text_analyzer::bow::*;
// use text_analyzer::bow_data_mapper;

// it should:
// - consume kafka event
// - build bow and save it to mongo (create or update)
// - compute similarities to other bow and store to neo4j
// - after update of last neo4j connections make query if exists documents without computed with this similarity and compute it
// - produce event 'text_analyzed'

fn main() {

    //     // let future = bow_data_mapper::create();
    //     // block_on(future);

    //     block_on(async_main());
    //     println!("Hello, world!");
}

// use futures::executor::block_on;

// use actix_web::{get, post, web, App, HttpResponse, HttpServer};

// use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

// const DB_NAME: &str = "myApp";
// const COLL_NAME: &str = "users";

// use serde::{Deserialize, Serialize};

// #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
// pub struct User {
//     pub first_name: String,
//     pub last_name: String,
//     pub username: String,
//     pub email: String,
// }
// /// Adds a new user to the "users" collection in the database.
// #[post("/add_user")]
// async fn add_user(client: web::Data<Client>) -> HttpResponse {
//     println!("{}", "u r here");

//     let collection = client.database(DB_NAME).collection(COLL_NAME);
//     let result = collection.insert_one(User::default()).await;
//     match result {
//         Ok(_) => HttpResponse::Ok().body("user added"),
//         Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//     }

//     // HttpResponse::Ok().body("user added")
// }

// /// Creates an index on the "username" field to force the values to be unique.
// async fn create_username_index(client: &Client) {
//     let options = IndexOptions::builder().unique(true).build();
//     let model = IndexModel::builder()
//         .keys(doc! { "username": 1 })
//         .options(options)
//         .build();
//     client
//         .database(DB_NAME)
//         .collection::<User>(COLL_NAME)
//         .create_index(model)
//         .await
//         .expect("creating an index should succeed");
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://root:example@localhost:27017/".into());

//     let client = Client::with_uri_str(uri).await.expect("failed to connect");
//     create_username_index(&client).await;

//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(client.clone()))
//             .service(add_user)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

// fn main() {

//     // let future = bow_data_mapper::create();
//     // block_on(future);

//     block_on(async_main());
//     println!("Hello, world!");
// }

// async fn async_main() {
//     let f1 = bow_data_mapper::create();
//     // let f2 = dance();

//     // `join!` is like `.await` but can wait for multiple futures concurrently.
//     // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
//     // future will take over the current thread. If `dance` becomes blocked,
//     // `learn_and_sing` can take back over. If both futures are blocked, then
//     // `async_main` is blocked and will yield to the executor.
//     futures::join!(f1);
// }
