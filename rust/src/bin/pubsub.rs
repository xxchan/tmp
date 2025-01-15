use std::time::{Duration, SystemTime};

use clap::{Parser, ValueEnum};
use futures::{stream::FuturesUnordered, StreamExt};
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::{
    client::{Client, ClientConfig},
    publisher::{self, PublisherConfig},
    subscription::{SeekTo, SubscribeConfig, SubscriptionConfig},
    topic::TopicConfig,
};

#[derive(clap::Parser, Debug)]
#[clap()]
struct Args {
    command: Command,
}
#[derive(Debug, Clone, ValueEnum)]
enum Command {
    Produce,
    Consume,
    CreateTopic,
    DeleteTopic,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    std::env::set_var("PUBSUB_EMULATOR_HOST", "localhost:5980");
    let client = Client::new(ClientConfig::default()).await.unwrap();

    let subscriptions = client.get_subscriptions(None).await.unwrap();
    let snapshots = client.get_snapshots(None).await.unwrap();
    println!(
        "subscriptions: {:#?}\nsnapshots: {:#?}",
        subscriptions, snapshots
    );

    match args.command {
        Command::CreateTopic => {
            match client
                .create_topic("topic-id", Some(TopicConfig::default()), None)
                .await
            {
                Ok(_) => {}
                Err(e) => println!("failed to create topic: {}", e),
            }

            match client
                .create_subscription(
                    "sub-id",
                    "topic-id",
                    SubscriptionConfig {
                        // retain-acked-messages must be set for seek to timestamp to work
                        retain_acked_messages: true,
                        ..Default::default()
                    },
                    None,
                )
                .await
            {
                Ok(_) => {}
                Err(e) => println!("failed to create subscription: {}", e),
            }
        }
        Command::DeleteTopic => {
            match client.topic("topic-id").delete(None).await {
                Ok(_) => {}
                Err(e) => println!("failed to delete topic: {}", e),
            }
            for sub in subscriptions {
                match sub.delete(None).await {
                    Ok(_) => {}
                    Err(e) => println!("failed to delete subscription: {}", e),
                }
            }
        }
        Command::Produce => {
            let topic = client.topic("topic-id");
            let publisher = topic.new_publisher(Some(PublisherConfig::default()));
            let a = publisher
                .publish(PubsubMessage {
                    data: "123".to_string().into_bytes(),
                    ..Default::default()
                })
                .await;
            a.get().await?;
            println!("published message")
        }
        Command::Consume => {
            let sub = subscriptions[0].clone();
            sub.ack(vec![
                "projects/local-project/subscriptions/sub-id:3".to_string()
            ])
            .await?;
            // sub.seek(SeekTo::Snapshot("123213".to_string()), None)
            //     .await?;
            // For seek to timestamp to work, retain-acked-messages must be set
            sub.seek(
                SeekTo::Timestamp(SystemTime::now() - Duration::new(1000, 0)),
                None,
            )
            .await?;
            let mut stream = sub.subscribe(Some(SubscribeConfig::default())).await?;
            while let Some(msg) = stream.next().await {
                println!("msg: {:?}", msg);

                // If do not ack, the message can be re consumed. seek is not necessary
                msg.ack().await?;

                // // If nack, the message will be repeatedly redelivered.
                // msg.nack().await?;
            }
        }
        Command::CreateTopic => todo!(),
    }

    Ok(())
}
