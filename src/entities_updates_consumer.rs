use crate::text_processor;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use kafka::error::Error as KafkaError;
use std::{thread, time::Duration};
use tokio::task::JoinSet;

pub async fn start() {
    let host = std::env::var("KAFKA_HOST").unwrap_or_else(|_| "localhost".into());
    let port = std::env::var("KAFKA_PORT").unwrap_or_else(|_| "9092".into());
    let topic = "entities_updates".to_owned();
    let group = "text_analyzer".to_owned();
    let broker = format!("{host}:{port}").to_owned();

    if let Err(e) = consume_messages(group, topic, vec![broker]).await {
        println!("Failed consuming messages: {}", e);
    }
}

// TODO: run it in multiple threads
async fn consume_messages(
    group: String,
    topic: String,
    brokers: Vec<String>,
) -> Result<(), KafkaError> {
    let mut con = Consumer::from_hosts(brokers)
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Earliest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()?;

    loop {
        let mss = con.poll()?;
        if mss.is_empty() {
            println!("No messages available right now. Sleep 1s");
            thread::sleep(Duration::from_millis(1000));
        }

        for ms in mss.iter() {
            let mut set = JoinSet::new();

            for m in ms.messages() {
                let json_string = String::from_utf8_lossy(&m.value);
                let parsed_entity: serde_json::Value = serde_json::from_str(&json_string).unwrap();

                let entity_id = parsed_entity["entity"]["id"].to_string();
                let text = parsed_entity["entity"]["content"].to_string();

                println!("Process: {:?}", parsed_entity);

                // TODO: set limit for concurent threads
                set.spawn(async move {
                    let processor = text_processor::Processor::init().await;
                    processor.call(text, entity_id).await;
                    // TODO: produce event "text analyzed"
                });
            }
            let _ = set.join_all().await;
            let _ = con.consume_messageset(ms);
        }

        con.commit_consumed()?;
    }
}
