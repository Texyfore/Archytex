use std::env;

use anyhow::{anyhow, Result};
use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    cameras::{perspective::PerspectiveCamera, jitter::JitterCamera},
    collector::array_collector::ArrayCollector,
    intersectables::bvh::BVH,
    loaders::{
        ascn::{amdl_textures, ASCNLoader},
        Loader,
    },
    renderers::{path_tracer::PathTracer},
    textures::{
        texture_repo::{self, TextureRepository},
        TextureID,
    },
};
use dotenv::dotenv;
use futures::StreamExt;
use lapin::{message::Delivery, Channel, Connection, ConnectionProperties};
use lru::LruCache;
use redis::AsyncCommands;

struct SceneData(BVH, JitterCamera<PerspectiveCamera>);

async fn render(
    texture_repo: &TextureRepository,
    cache: &mut LruCache<String, SceneData>,
    redis_client: &mut redis::Client,
    channel: &Channel,
    delivery: Delivery,
) -> Result<()> {
    println!("Rendering");
    let s = String::from_utf8(delivery.data)?;
    let s: Vec<&str> = s.split('#').collect();
    let task = s[0].to_string();
    let id = s[1].to_string();
    let response = s[2].to_string();
    let width: usize = redis::Cmd::get(format!("archyrt:{}:width", task)).query(redis_client)?;
    let height: usize = redis::Cmd::get(format!("archyrt:{}:height", task)).query(redis_client)?;
    let scene = match cache.get(&task) {
        Some(a) => a,
        None => {
            let scene: Vec<u8> =
                redis::Cmd::get(format!("archyrt:{}:scene", task)).query(redis_client)?;
            let scene = ASCNLoader::from_bytes(&scene)?;
            let bvh = BVH::from_triangles(scene.get_triangles())
                .ok_or_else(|| anyhow!("Unable to create BVH"))?;
                let camera = scene.get_camera().clone();
                let camera = JitterCamera::new(camera, width, height);
            let data = SceneData(bvh, camera);
            cache.put(task.clone(), data);
            cache.get(&task).unwrap()
        }
    };
    let renderer = PathTracer {
        camera: &scene.1,
        object: &scene.0,
        bounces: 5,
        skybox: Some(TextureID::new(&"skybox")),
    };
    let image = ArrayCollector {}.collect(renderer, texture_repo, width, height);
    //Convert image into bytes
    let image: Vec<u8> = image
        .into_iter()
        .flatten()
        .map(|vec| vec.inner)
        .flatten()
        .map(|b| (b as f32).to_le_bytes())
        .flatten()
        .collect();
    let temp = format!("archyrt:temp:{}", id);
    let image_key = format!("archyrt:{}:image", task);
    let channel = channel.clone();
    let redis_client = redis_client.clone();

    async_global_executor::spawn(async move {
        let mut con = redis_client.get_async_connection().await.unwrap();
        //Upload image to Redis
        let _: () = redis::cmd("AI.TENSORSET")
            .arg(&temp)
            .arg("FLOAT")
            .arg(width)
            .arg(height)
            .arg(3)
            .arg("BLOB")
            .arg(image)
            .query_async(&mut con)
            .await
            .unwrap();
        //Add image to accumulator
        let _: () = redis::cmd("AI.SCRIPTEXECUTE")
            .arg("archyrt:scripts")
            .arg("add")
            .arg("INPUTS")
            .arg(2)
            .arg(&temp)
            .arg(&image_key)
            .arg("OUTPUTS")
            .arg(1)
            .arg(&image_key)
            .query_async(&mut con)
            .await
            .unwrap();
        //Remove temporary storage
        let _: () = redis::Cmd::del(temp).query_async(&mut con).await.unwrap();
        println!("Sending ACK");
        channel
            .basic_publish(
                "",
                &response,
                Default::default(),
                Default::default(),
                Default::default(),
            )
            .await
            .unwrap();
        channel
            .basic_ack(delivery.delivery_tag, Default::default())
            .await
            .unwrap();
    })
    .detach();
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

        let mut textures = TextureRepository::new();
        amdl_textures::load_into(&mut textures, "../assets")?;
        texture_repo::exr::load_into(
            &mut textures,
            "../assets",
            &[(TextureID::new(&"skybox"), "skybox.exr")],
        )?;

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
            if let Err(err) = future.await {
                println!("Error: {}", err);
            }
        }
        Ok(())
    })
}
