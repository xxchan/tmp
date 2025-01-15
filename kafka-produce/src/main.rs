use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use serde_json::json;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Kafka configuration
    let producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("queue.buffering.max.messages", "1000000")
        .set("queue.buffering.max.ms", "1000")
        .set("batch.num.messages", "100000")
        .set("linger.ms", "5")
        .create()?;

    let topic = "your_topic_name"; // Replace with your desired topic name

    let num_messages = 10_000_000;
    let num_partitions = 6;

    let start_time = Instant::now();

    for i in 1..=num_messages {
        let s = format!("s{}", i).repeat(10);
        let data = json!({ "x": i, "s": s }).to_string();
        let record = BaseRecord::to(topic)
            .partition((i % num_partitions) as i32)
            .payload(&data)
            .key(&s);
        producer.send(record).unwrap();

        if i % 100_000 == 0 {
            println!("Produced {} messages", i);
        }

        producer.poll(std::time::Duration::from_millis(0));
    }

    // Wait for any outstanding messages to be delivered
    producer.flush(std::time::Duration::from_secs(60))?;

    let end_time = Instant::now();
    let total_time = end_time.duration_since(start_time);
    println!(
        "Produced {} messages in {:.2} seconds",
        num_messages,
        total_time.as_secs_f64()
    );
    println!(
        "Average throughput: {:.2} messages/second",
        num_messages as f64 / total_time.as_secs_f64()
    );

    Ok(())
}
