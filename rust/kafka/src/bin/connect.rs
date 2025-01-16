use clap::{Parser, ValueEnum};
use itertools::Itertools;
use log::{info, warn};

use rdkafka::admin::{AdminClient, NewTopic, TopicReplication};
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
struct Args {}

const TOPIC: &str = "x";

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
#[derive(Clone)]
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

async fn connect() -> anyhow::Result<()> {
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
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "smallest")
        .set("isolation.level", "read_committed");
    // set SSL
    if std::env::var(&"USE_SASL").is_ok() {
        config
            .set("security.protocol", "SASL_SSL")
            .set("sasl.mechanisms", std::env::var(&"SASL_MECHANISM").unwrap_or("PLAIN".to_string()))
            .set("sasl.username", std::env::var(&"USERNAME").unwrap())
            .set("sasl.password", std::env::var(&"PASSWORD").unwrap());
    }

    tracing::info!("admin creation start");
    let admin: AdminClient<CustomContext> =
        config.create_with_context(context.clone()).await.unwrap();
    tracing::info!("admin created");
    let new_topic = NewTopic::new("T", 1, TopicReplication::Fixed(2));
    admin
        .create_topics([&new_topic], &Default::default())
        .await
        .unwrap();
    tracing::info!("admin created topics");

    tracing::info!("consumer creation start");
    let consumer: LoggingConsumer = config
        .create_with_context(context)
        .await
        .expect("Consumer creation failed");
    tracing::info!("consumer created");

    let metadata = consumer
        .fetch_metadata(Some("xxtopic"), Duration::from_secs(10))
        .await
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

    Ok(())
}

use core::time;
use std::thread::sleep;
use std::time::Duration;

use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    dotenv::dotenv().ok();
    env_logger::init();

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    for i in 0..10000 {
        tracing::info!("create consumer {}", i);
        connect().await.unwrap();
    }
}
