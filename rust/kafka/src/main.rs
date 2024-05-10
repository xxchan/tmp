use clap::{Parser, ValueEnum};
use itertools::Itertools;
use log::{info, warn};

use rdkafka::client::ClientContext;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{CommitMode, Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::message::{Headers, Message};
use rdkafka::topic_partition_list::TopicPartitionList;
use rdkafka::util::get_rdkafka_version;
use rdkafka::Offset;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap()]
struct Args {
    mode: ConsumerMode,
}

const TOPIC: &str = "x";

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance<'_>) {
        info!("Pre rebalance: {:#?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance<'_>) {
        info!("Post rebalance: {:#?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

#[derive(ValueEnum, Clone, Debug)]
enum ConsumerMode {
    Subscribe,
    Assign,
}

async fn consume_and_print(mode: ConsumerMode) -> anyhow::Result<()> {
    let context = CustomContext;

    let mut config = ClientConfig::new();
    config
        .set("group.id", "not-exists")
        .set(
            "bootstrap.servers",
            std::env::var(&"BOOTSTRAP_SERVERS").unwrap(),
        )
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        // earliest
        .set("auto.offset.reset", "earliest");
    // set SSL
    if std::env::var(&"USE_SASL").is_ok() {
        config
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", "PLAIN")
            .set("sasl.username", std::env::var(&"USERNAME").unwrap())
            .set("sasl.password", std::env::var(&"PASSWORD").unwrap());
    }
    let consumer: LoggingConsumer = config
        .create_with_context(context)
        .expect("Consumer creation failed");

    let metadata = consumer
        .fetch_metadata(Some(TOPIC), Duration::from_secs(10))
        .unwrap();
    warn!(
        "metadata {}",
        metadata
            .topics()
            .iter()
            .format_with(",", |topic, f| f(&format!(
                "{}: {}",
                topic.name(),
                topic
                    .partitions()
                    .iter()
                    .format_with(",", |partition, f| f(&format!(
                        "(partition: {}, leader: {}, replicas: {:?})",
                        partition.id(),
                        partition.leader(),
                        partition.replicas()
                    )))
            )))
    );

    match mode {
        ConsumerMode::Subscribe => {
            // SUBSCRIBE - consumer group and auto rebalance

            consumer
                .subscribe(&[TOPIC].to_vec())
                .expect("Can't subscribe to specified topics");
        }
        ConsumerMode::Assign => {
            // ASSIGN - manual assignment
            let mut tpl = TopicPartitionList::new();
            // = add_partition + set_partition_offset
            // tpl.add_partition_offset("mytopic", 0, Offset::Beginning)
            //     .unwrap();
            // tpl.add_partition_offset("mytopic", 1, Offset::Beginning)
            //     .unwrap();
            // tpl.add_partition_offset("mytopic", 2, Offset::Beginning)
            //     .unwrap();
            tpl.add_partition_offset(TOPIC, 0, Offset::End)?;

            //  Duplicate mytopic [1] in input list
            // tpl.add_partition_offset("mytopic", 1, Offset::Beginning).unwrap();

            // this don't fail (e.g., if the partition doesn't exist)
            // When add parition without offset (Offset::Invalid?), it seems to be starting from beginning
            // tpl.add_partition(TOPIC, 0);

            // tpl.add_partition("topic2", 0);
            // tpl.add_partition("topic2", 1);

            // tpl.set_partition_offset("topic1", 0, Offset::Offset(0))
            //     .unwrap();
            // tpl.set_partition_offset("topic1", 1, Offset::Offset(1))
            //     .unwrap();
            // tpl.set_partition_offset("topic2", 0, Offset::Offset(2))
            //     .unwrap();
            // tpl.set_partition_offset("topic2", 1, Offset::Offset(3))
            //     .unwrap();
            consumer.assign(&tpl).unwrap();
        }
    }

    loop {
        warn!("loop");
        match consumer.recv().await {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(m) => {
                let payload = match m.payload_view::<str>() {
                    None => "",
                    Some(Ok(s)) => s,
                    Some(Err(e)) => {
                        warn!("Error while deserializing message payload: {:?}", e);
                        ""
                    }
                };
                info!("key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                      m.key(), payload, m.topic(), m.partition(), m.offset(), m.timestamp());
                if let Some(headers) = m.headers() {
                    for header in headers.iter() {
                        info!("  Header {:#?}: {:?}", header.key, header.value);
                    }
                }
                // consumer.commit_message(&m, CommitMode::Sync).unwrap();
                // return
            }
        };
    }
}

use core::time;
use std::thread::sleep;
use std::time::Duration;

use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};

async fn produce() {
    let producer: &FutureProducer = &ClientConfig::new()
        .set(
            "bootstrap.servers",
            std::env::var(&"BOOTSTRAP_SERVERS").unwrap(),
        )
        .set("message.timeout.ms", "5000")
        // set SSL
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", std::env::var(&"USERNAME").unwrap())
        .set("sasl.password", std::env::var(&"PASSWORD").unwrap())
        .create()
        .expect("Producer creation error");

    // This loop is non blocking: all messages will be sent one after the other, without waiting
    // for the results.
    let futures = (0..1000000)
        .map(|i| async move {
            // The send operation on the topic returns a future, which will be
            // completed once the result or failure from Kafka is received.
            let delivery_status = producer
                .send(
                    FutureRecord::to(TOPIC)
                        .payload(&format!("Message {}", i))
                        .key(&format!("Key {}", i))
                        .headers(OwnedHeaders::new().insert(Header {
                            key: "header_key",
                            value: Some("header_value"),
                        })),
                    Duration::from_secs(0),
                )
                .await;

            // This will be executed when the result is received.
            info!("Delivery status for message {} received", i);
            delivery_status
        })
        .collect::<Vec<_>>();

    // This loop will wait until all delivery statuses have been received.

    // join all futures
    futures::future::join_all(futures).await;
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    dotenv::dotenv().ok();
    env_logger::init();

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    // produce().await;
    consume_and_print(args.mode).await.unwrap();
}
