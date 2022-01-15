use std::{env, fmt::format};

use anyhow::Result;
use dotenv::dotenv;
use futures::TryFutureExt;
use futures_util::stream::StreamExt;
use lapin::{
    message::Delivery,
    options::{BasicConsumeOptions, QueueDeclareOptions, BasicAckOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Queue,
};

async fn handle_job(
    mut redis_client: redis::Client,
    channel: Channel,
    delivery: Delivery,
    response_queue: Queue,
    task_queue: Queue,
) -> Result<()> {
    let response_queue = response_queue.name().as_str();
    let task_queue = task_queue.name().as_str();
    let s = String::from_utf8(delivery.data)?;
    let samples: i32 = redis::Cmd::get(format!("archyrt:{}:samples", s)).query(&mut redis_client)?;
    let payload = s.clone();
    let payload = payload.into_bytes();
    //Put as many tasks on the queue as there are samples
    println!("Sending...");
    futures::stream::iter(0..samples).for_each(|_|async {
        channel.clone().basic_publish("", task_queue, Default::default(), payload.clone(), Default::default()).await.unwrap();
        ()
    }).await;
    channel.basic_ack(delivery.delivery_tag, Default::default()).await?;
    println!("Done!");
    let consumer_tag = format!("consumer_{}", s);
    let mut consumer = channel.basic_consume(response_queue, &consumer_tag, BasicConsumeOptions{
        no_ack: true,
        ..Default::default()
    }, Default::default()).await?;
    let mut counter = 0;
    while let Some(delivery) = consumer.next().await {
        let (_, delivery) = delivery.unwrap();
        counter += 1;
        if counter >= samples{
            break;
        }
    }
    channel.queue_delete(response_queue, Default::default()).await?;
    //TODO: Combine results
    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();
    println!("Hello, world!");

    let amqp_addr = env::var("AMQP_ADDR").unwrap();
    let redis_addr = env::var("REDIS_ADDR").unwrap();
    async_global_executor::block_on(async {
        let rabbitmq_client = Connection::connect(
            &amqp_addr,
            ConnectionProperties::default().with_default_executor(8),
        )
        .await?;
        let redis_client = redis::Client::open(redis_addr)?;

        let channel = rabbitmq_client.create_channel().await?;

        let queue = channel
            .queue_declare("archyrt:dispatch", Default::default(), Default::default())
            .await?;
        let task_queue = channel
            .queue_declare("archyrt:taskqueue", Default::default(), Default::default())
            .await?;

        let mut consumer = channel
            .basic_consume(
                queue.name().as_str(),
                "archyrt:consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        while let Some(delivery) = consumer.next().await {
            let (_, delivery) = delivery.unwrap();
            let response_queue = channel
                .queue_declare("", QueueDeclareOptions {
                    exclusive: true,
                    ..Default::default()
                }, Default::default())
                .await?;
            async_global_executor::spawn(handle_job(
                redis_client.clone(),
                channel.clone(),
                delivery,
                response_queue,
                task_queue.clone(),
            ))
            .detach();
        }

        Ok(())
    })
}
