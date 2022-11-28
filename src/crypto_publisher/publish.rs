use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::message::OwnedHeaders;
use log::info;

use std:: time::Duration;

#[tokio::main]
pub async fn publish(broker: &str, topic: &str, pub_message: &str, count: i32) {
    
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", broker)
        .set("message.timeout.ms", "5000")
        .set("security.protocol", "plaintext")
        .create()
        .expect("Failed to create producer");

        let payload = format!("message {}", pub_message);
        let key = format!("key {}", count);

        info!("Sending message '{}'", count);

        let status = producer.send(
            FutureRecord::to(topic)
                .payload(&payload)
                .key(&key)
                .headers(OwnedHeaders::new().add(
                    &format!("header_key_{}", count),
                    &format!("header_value_{}", count)
                )),
            Duration::from_secs(0)
        ).await;

        info!("Status '{:?}' received from message '{}'", status, count);
}