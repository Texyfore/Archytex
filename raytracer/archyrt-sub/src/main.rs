use std::env;

use anyhow::{anyhow, Result};
use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    cameras::perspective::PerspectiveCamera,
    collector::array_collector::ArrayCollector,
    intersectables::bvh::BVH,
    loaders::{
        amdl::{amdl_textures, AMDLLoader},
        Loader,
    },
    renderers::basic_renderer::BasicRenderer,
    textures::texture_repo::png::PngTextureRepo,
    vector,
};
use dotenv::dotenv;
use futures::StreamExt;
use lapin::{message::Delivery, Channel, Connection, ConnectionProperties};
use lru::LruCache;

struct SceneData(BVH, PerspectiveCamera);

async fn render(
    texture_repo: &PngTextureRepo,
    cache: &mut LruCache<String, SceneData>,
    redis_client: &mut redis::Client,
    channel: &Channel,
    delivery: Delivery,
) -> Result<()> {
    println!("Rendering");
    let s = String::from_utf8(delivery.data)?;
    let s: Vec<&str> = s.split("#").collect();
    let task = s[0].to_string();
    let id = s[1].to_string();
    let response = s[2].to_string();
    let scene = match cache.get(&task) {
        Some(a) => a,
        None => {
            let scene: Vec<u8> =
                redis::Cmd::get(format!("archyrt:{}:scene", task)).query(redis_client)?;
            let scene = AMDLLoader::from_bytes(&scene)?;
            let bvh = BVH::from_triangles(scene.get_triangles())
                .ok_or_else(|| anyhow!("Unable to create BVH"))?;
            let data = SceneData(bvh, scene.get_camera().clone());
            cache.put(task.clone(), data);
            cache.get(&task).unwrap()
        }
    };
    let width: usize = redis::Cmd::get(format!("archyrt:{}:width", task)).query(redis_client)?;
    let height: usize = redis::Cmd::get(format!("archyrt:{}:height", task)).query(redis_client)?;
    let renderer = BasicRenderer {
        camera: &scene.1,
        object: &scene.0,
        lamp: vector![1.0, 1.0, 1.0],
    };
    let image = ArrayCollector {}.collect(renderer, texture_repo, width, height);
    //Convert image into bytes
    let image: Vec<u8> = image
        .into_iter()
        .flatten()
        .map(|vec| vec.inner)
        .flatten()
        .map(|b| b.to_le_bytes())
        .flatten()
        .collect();
    let temp = format!("archyrt:temp:{}", id);
    let image_key = format!("archyrt:{}:image", task);
    //Upload image to Redis
    redis::cmd("AI.TENSORSET")
        .arg(&temp)
        .arg("DOUBLE")
        .arg(width)
        .arg(height)
        .arg(3)
        .arg("BLOB")
        .arg(image)
        .query(redis_client)?;
    //Add image to accumulator
    redis::cmd("AI.SCRIPTEXECUTE")
        .arg("archyrt:scripts")
        .arg("add")
        .arg("INPUTS")
        .arg(2)
        .arg(&temp)
        .arg(&image_key)
        .arg("OUTPUTS")
        .arg(1)
        .arg(&image_key)
        .query(redis_client)?;
    //Remove temporary storage
    redis::Cmd::del(temp).query(redis_client)?;
    println!("Sending ACK");
    channel
        .basic_publish(
            "",
            &response,
            Default::default(),
            Default::default(),
            Default::default(),
        )
        .await?;
    channel
        .basic_ack(delivery.delivery_tag, Default::default())
        .await?;
    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();
    println!("Hello, world!");

    let amqp_addr = env::var("AMQP_ADDR").unwrap();
    let redis_addr = env::var("REDIS_ADDR").unwrap();
    async_global_executor::block_on(async {
        let mut cache: LruCache<String, SceneData> = LruCache::new(5);
        let rabbitmq_client = Connection::connect(
            &amqp_addr,
            ConnectionProperties::default().with_default_executor(8),
        )
        .await?;
        let mut redis_client = redis::Client::open(redis_addr)?;

        let textures = amdl_textures::load("../assets")?;

        let channel = rabbitmq_client.create_channel().await?;
        let task_queue = channel
            .queue_declare("archyrt:taskqueue", Default::default(), Default::default())
            .await?;
        let mut consumer = channel
            .basic_consume(
                task_queue.name().as_str(),
                "consumer_TODONUMBER",
                Default::default(),
                Default::default(),
            )
            .await?;
        while let Some(delivery) = consumer.next().await {
            let (_, delivery) = delivery.unwrap();
            let future = render(&textures, &mut cache, &mut redis_client, &channel, delivery);
            if let Err(err) = future.await{
                println!("Error: {}", err);
            }
        }
        Ok(())
    })
}
