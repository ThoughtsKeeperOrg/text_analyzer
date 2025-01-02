use neo4rs::*;

pub async fn prepare() {
    Client::init()
    .await
    .create_constraints()
    .await;
}

pub struct Client {
    graph: Graph,
}

impl Client {
    pub async fn init() -> Self {
        let host = std::env::var("NEO4J_HOST").unwrap_or_else(|_| "localhost".into());
        let port = std::env::var("NEO4J_PORT").unwrap_or_else(|_| "7687".into());
        let user = std::env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".into());
        let password = std::env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "your_password".into());
        let db = std::env::var("NEO4J_DB").unwrap_or_else(|_| "neo4j".into());
        let config = ConfigBuilder::default()
            .uri(format!("{host}:{port}"))
            .user(user)
            .password(password)
            .db(db)
            .build()
            .unwrap();

        Self {
            graph: Graph::connect(config).await.unwrap(),
        }
    }

    pub async fn find(&self, entity_id: &String) -> Option<Row> {
        self.graph
            .execute(
                query("MATCH (t:Thought) WHERE t.entity_id = $entity_id RETURN t")
                    .param("entity_id", entity_id.clone()),
            )
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
    }

    pub async fn find_relation(&self, a_entity_id: &String, b_entity_id: &String) -> Option<Row> {
        self.graph
            .execute(query("MATCH (a:Thought { entity_id: $a_entity_id })-[relation:similarity]-(b:Thought { entity_id: $b_entity_id }) RETURN relation")
                .param("a_entity_id", a_entity_id.clone())
                .param("b_entity_id", b_entity_id.clone())
            )
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
    }

    pub async fn create(&self, entity_id: &String) -> Result<(), Error> {
        self.graph
            .run(
                query("CREATE (t:Thought { entity_id: $entity_id })")
                    .param("entity_id", entity_id.clone()),
            )
            .await
    }

    pub async fn create_similarity_relation(
        &self,
        a_entity_id: &String,
        b_entity_id: &String,
        similarity: f32,
    ) {
        if self.find(&a_entity_id).await.is_none() {
            let _ = self.create(&a_entity_id).await;
        }

        if self.find(&b_entity_id).await.is_none() {
            let _ = self.create(&b_entity_id).await;
        }

        if self
            .find_relation(&a_entity_id, &b_entity_id)
            .await
            .is_none()
        {
            self.graph
                .run(
                    query(
                        "MATCH (a:Thought { entity_id: $a_entity_id })
                       MATCH (b:Thought { entity_id: $b_entity_id })
                       CREATE (a)-[r:similarity {estimation: $similarity}]->(b) RETURN r",
                    )
                    .param("a_entity_id", a_entity_id.clone())
                    .param("b_entity_id", b_entity_id.clone())
                    .param("similarity", similarity),
                )
                .await
                .unwrap();
        } else {
            self.graph
                .run(
                    query(
                        "MATCH (a:Thought { entity_id: $a_entity_id })
                       MATCH (b:Thought { entity_id: $b_entity_id })
                       MATCH (a)-[r:similarity]-(b) 
                       SET r.estimation =  $similarity
                       RETURN r",
                    )
                    .param("a_entity_id", a_entity_id.clone())
                    .param("b_entity_id", b_entity_id.clone())
                    .param("similarity", similarity),
                )
                .await
                .unwrap();
        }
    }

    pub async fn count_by_entity_id(&self, entity_id: &String) -> usize {
        self.graph
            .execute(
                query("MATCH (n:Thought { entity_id: $entity_id }) RETURN COUNT(n) AS n")
                    .param("entity_id", entity_id.clone()),
            )
            .await
            .unwrap()
            .next()
            .await
            .unwrap()
            .unwrap()
            .get::<usize>("n")
            .unwrap()
    }

    pub async fn delete_all(&self) {
        self.graph
            .run(query("MATCH (n) DETACH DELETE n"))
            .await
            .unwrap();
    }

    pub async fn find_missing_relations(&self, entity_id: &String) -> Vec<String> {
        let mut ids = vec![];
        let mut rows = self
            .graph
            .execute(
                query(
                    "MATCH (a:Thought { entity_id: $entity_id })
                            MATCH (node) 
                            WHERE NOT (a)-[:similarity]-(node)
                            RETURN node",
                )
                .param("entity_id", entity_id.clone()),
            )
            .await
            .unwrap();

        while let Ok(Some(row)) = rows.next().await {
            let relation: Node = row.get("node").unwrap();
            let missing_id = relation.get::<String>("entity_id").unwrap();
            if missing_id != *entity_id {
                ids.push(missing_id);
            }
        }

        ids
    }

    pub async fn get_similarity_estimation(
        &self,
        a_entity_id: &String,
        b_entity_id: &String,
    ) -> f32 {
        let row = self
            .find_relation(&a_entity_id, &b_entity_id)
            .await
            .unwrap();
        let relation: Relation = row.get("relation").unwrap();
        relation.get::<f32>("estimation").unwrap()
    }

    pub async fn create_constraints(&self) {
        let result = self
            .graph
            .run(query(
                "CREATE CONSTRAINT thought_entity_id FOR (t:Thought) REQUIRE t.entity_id IS UNIQUE",
            ))
            .await;

        match result {
            Ok(_) => println!("Neo4j constraints are added."),
            Err(error) => {
                if error
                    .to_string()
                    .contains("Neo.ClientError.Schema.EquivalentSchemaRuleAlreadyExists")
                {
                    println!("Neo4j constraints already exists.")
                } else {
                    panic!("Neo4j constraints creation error: {error:?}");
                }
            }
        }
    }
}

use serial_test::serial;

#[tokio::test]
async fn test_connect_to_db() {
    let client = Client::init().await;

    let mut result = client.graph.execute(query("RETURN 1")).await.unwrap();
    let row = result.next().await.unwrap().unwrap();
    let value: i64 = row.get("1").unwrap();
    assert_eq!(1, value);
    assert!(result.next().await.unwrap().is_none());
}

#[tokio::test]
#[serial]
async fn test_uniqueness_constraint() {
    let client = Client::init().await;
    let _ = client.delete_all().await;
    let _ = client.create_constraints().await;

    let entity_id = "999".to_string();

    assert!(client.create(&entity_id).await.is_ok());
    assert!(client.create(&entity_id).await.is_err());
    let _ = client.delete_all().await;
}

#[tokio::test]
#[serial]
async fn test_crud() {
    let client = Client::init().await;
    let _ = client.create_constraints();
    let _ = client.delete_all().await;

    let entity_id = "1111".to_string();
    let count = client.count_by_entity_id(&entity_id).await;
    assert_eq!(count, 0);
    let _ = client.create(&entity_id).await;
    let count = client.count_by_entity_id(&entity_id).await;
    assert_eq!(count, 1);
    let entity_in_db = client.find(&entity_id).await;
    assert_eq!(entity_in_db.is_some(), true);
    let _ = client.delete_all().await;
    let entity_in_db = client.find(&entity_id).await;
    assert_eq!(entity_in_db.is_some(), false);
    let count = client.count_by_entity_id(&entity_id).await;
    assert_eq!(count, 0);
}

#[tokio::test]
#[serial]
async fn test_create_similarity_relation() {
    let client = Client::init().await;
    let _ = client.create_constraints().await;
    let _ = client.delete_all().await;

    let a_entity_id = "1111".to_string();
    let b_entity_id = "2222".to_string();
    let similarity_estimation: f32 = 0.33;
    let count = client.count_by_entity_id(&a_entity_id).await;
    assert_eq!(count, 0);
    let count = client.count_by_entity_id(&b_entity_id).await;
    assert_eq!(count, 0);

    let _ = client
        .create_similarity_relation(&a_entity_id, &b_entity_id, similarity_estimation)
        .await;

    let count = client.count_by_entity_id(&a_entity_id).await;
    assert_eq!(count, 1);
    let count = client.count_by_entity_id(&b_entity_id).await;
    assert_eq!(count, 1);

    let estimation_from_db = client
        .get_similarity_estimation(&a_entity_id, &b_entity_id)
        .await;
    assert_eq!(estimation_from_db, 0.33);

    let _ = client
        .create_similarity_relation(&a_entity_id, &b_entity_id, 0.7)
        .await;

    let count = client.count_by_entity_id(&a_entity_id).await;
    assert_eq!(count, 1);
    let count = client.count_by_entity_id(&b_entity_id).await;
    assert_eq!(count, 1);

    let estimation_from_db = client
        .get_similarity_estimation(&a_entity_id, &b_entity_id)
        .await;
    assert_eq!(estimation_from_db, 0.7);

    let _ = client.delete_all().await;
}

#[tokio::test]
#[serial]
async fn test_find_missing_relations() {
    let client = Client::init().await;
    let _ = client.create_constraints().await;
    let _ = client.delete_all().await;

    let a_entity_id = "11111".to_string();
    let b_entity_id = "22222".to_string();
    let c_entity_id = "33333".to_string();
    let d_entity_id = "44444".to_string();
    let e_entity_id = "55555".to_string();
    let _ = client.create(&a_entity_id).await;
    let _ = client.create(&b_entity_id).await;
    let _ = client.create(&c_entity_id).await;
    let _ = client.create(&d_entity_id).await;
    let _ = client.create(&e_entity_id).await;

    let missing_ids = client.find_missing_relations(&a_entity_id).await;

    assert!(!missing_ids.contains(&a_entity_id));
    assert!(missing_ids.contains(&b_entity_id));
    assert!(missing_ids.contains(&c_entity_id));
    assert!(missing_ids.contains(&d_entity_id));
    assert!(missing_ids.contains(&e_entity_id));

    let _ = client
        .create_similarity_relation(&a_entity_id, &b_entity_id, 0.1)
        .await;
    let _ = client
        .create_similarity_relation(&c_entity_id, &d_entity_id, 0.1)
        .await;

    let missing_ids = client.find_missing_relations(&a_entity_id).await;

    assert!(!missing_ids.contains(&a_entity_id));
    assert!(!missing_ids.contains(&b_entity_id));
    assert!(missing_ids.contains(&c_entity_id));
    assert!(missing_ids.contains(&d_entity_id));
    assert!(missing_ids.contains(&e_entity_id));
}
