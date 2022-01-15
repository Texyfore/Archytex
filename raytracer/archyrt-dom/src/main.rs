use std::{env, fmt::format, path::Path};

use anyhow::Result;
use dotenv::dotenv;
use futures::TryFutureExt;
use futures_util::stream::StreamExt;
use image::Rgb;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Queue,
};
use uuid::Uuid;

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
    let samples: i32 =
        redis::Cmd::get(format!("archyrt:{}:samples", s)).query(&mut redis_client)?;
    let width: usize = redis::Cmd::get(format!("archyrt:{}:width", s)).query(&mut redis_client)?;
    let height: usize =
        redis::Cmd::get(format!("archyrt:{}:height", s)).query(&mut redis_client)?;
    //Create image storage on Redis
    let image_key = format!("archyrt:{}:image", s);
    redis::cmd("AI.TENSORSET")
        .arg(&image_key)
        .arg("DOUBLE")
        .arg(width)
        .arg(height)
        .arg(3)
        .query(&mut redis_client)?;
    let payload = s.clone();
    //Put as many tasks on the queue as there are samples
    futures::stream::iter(0..samples)
        .for_each(|_| async {
            let id = Uuid::new_v4();
            let payload = format!("{}#{}#{}", payload, id, response_queue);
            let payload = payload.into_bytes();
            channel
                .clone()
                .basic_publish(
                    "",
                    task_queue,
                    Default::default(),
                    payload,
                    Default::default(),
                )
                .await
                .unwrap();
            ()
        })
        .await;
    channel
        .basic_ack(delivery.delivery_tag, Default::default())
        .await?;
    let consumer_tag = format!("consumer_{}", s);
    let mut consumer = channel
        .basic_consume(
            response_queue,
            &consumer_tag,
            BasicConsumeOptions {
                no_ack: true,
                ..Default::default()
            },
            Default::default(),
        )
        .await?;
    let mut counter = 0;
    while let Some(delivery) = consumer.next().await {
        let (_, delivery) = delivery.unwrap();
        counter += 1;
        if counter >= samples {
            break;
        }
    }
    channel
        .queue_delete(response_queue, Default::default())
        .await?;
    redis::cmd("AI.SCRIPTEXECUTE")
        .arg("archyrt:scripts")
        .arg("divide")
        .arg("INPUTS")
        .arg(1)
        .arg(&image_key)
        .arg("ARGS")
        .arg(1)
        .arg(counter)
        .arg("OUTPUTS")
        .arg(1)
        .arg(&image_key)
        .query(&mut redis_client)?;
    let image: Vec<u8> = redis::cmd("AI.TENSORGET")
        .arg(&image_key)
        .arg("BLOB")
        .query(&mut redis_client)?;
    let image: Vec<f64> = image
        .chunks(64 / 8)
        .map(|a| {
            let a: [u8; 8] = a.try_into().unwrap();
            f64::from_le_bytes(a)
        })
        .collect();

    let image: Vec<[f64; 3]> = image.chunks(3).map(|a| a.try_into().unwrap()).collect();
    let image: Vec<&[[f64; 3]]> = image.chunks(width).collect();
    let mut output = image::RgbImage::new(width as u32, height as u32);
    for (x, y, pixel) in output.enumerate_pixels_mut() {
        let [r, g, b] = image[y as usize][x as usize];
        let r = (r * 255.0).clamp(0.0, 255.0) as u8;
        let g = (g * 255.0).clamp(0.0, 255.0) as u8;
        let b = (b * 255.0).clamp(0.0, 255.0) as u8;
        *pixel = Rgb([r, g, b]);
    }
    let path = Path::new(&env::var("IMAGES").unwrap())
        .join(s)
        .with_extension("png");
    output.save(path)?;
    redis::Cmd::del(&image_key).query(&mut redis_client)?;
    Ok(())
}

static TORCHSCRIPT: &str = "
def add(tensors: List[Tensor], keys: List[str], args: List[str]):
    return tensors[0]+tensors[1]

def divide(tensors: List[Tensor], keys: List[str], args: List[str]):
    return tensors[0]/int(args[0])
";

fn main() -> Result<()> {
    dotenv().ok();
    println!("Hello, world!");

    let amqp_addr = env::var("AMQP_ADDR").unwrap();
    let redis_addr = env::var("REDIS_ADDR").unwrap();
    env::var("IMAGES").unwrap();
    async_global_executor::block_on(async {
        let rabbitmq_client = Connection::connect(
            &amqp_addr,
            ConnectionProperties::default().with_default_executor(8),
        )
        .await?;
        let mut redis_client = redis::Client::open(redis_addr)?;
        redis::cmd("AI.SCRIPTSTORE")
            .arg("archyrt:scripts")
            .arg("CPU")
            .arg("ENTRY_POINTS")
            .arg(2)
            .arg("add")
            .arg("divide")
            .arg("SOURCE")
            .arg(TORCHSCRIPT)
            .query(&mut redis_client)?;

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
                .queue_declare(
                    "",
                    QueueDeclareOptions {
                        exclusive: true,
                        ..Default::default()
                    },
                    Default::default(),
                )
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
