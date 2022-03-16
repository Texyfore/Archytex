mod shifted_view;

use std::env;

use anyhow::{anyhow, Result};
use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    cameras::{perspective::PerspectiveCamera, jitter::JitterCamera},
    collector::array_collector::ArrayCollector,
    intersectables::bvh::BVH,
    loaders::{
        ascn::{amdl_textures, ASCNLoader},
        Loader, amdl::{repo::{PropRequest, PropRepository}, self},
    },
    renderers::{path_tracer::PathTracer},
    textures::{
        texture_repo::{self, TextureRepository},
        TextureID,
    }, utilities::ray::Intersectable,
};
use dotenv::dotenv;
use futures::StreamExt;
use lapin::{message::Delivery, Channel, Connection, ConnectionProperties};
use lru::LruCache;
use redis::AsyncCommands;

use crate::shifted_view::ShiftedView;

struct SceneData(BVH, JitterCamera<PerspectiveCamera>, Vec<PropRequest>);

async fn render(
    texture_repo: &TextureRepository,
    prop_repo: &PropRepository,
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
    let x: usize = s[3].parse()?;
    let y: usize = s[4].parse()?;
    let width: usize = redis::Cmd::get(format!("archyrt:{}:width", task)).query(redis_client)?;
    let height: usize = redis::Cmd::get(format!("archyrt:{}:height", task)).query(redis_client)?;
    let part_width = width/4;
    let part_height = height/4;
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
            let prop_requests = scene.get_prop_requests().clone();
            let data = SceneData(bvh, camera, prop_requests);
            cache.put(task.clone(), data);
            cache.get(&task).unwrap()
        }
    };
    let props = prop_repo.fulfill_all(&scene.2)?;
    let object = &scene.0;
    let object = object.union(props);
    let renderer = PathTracer {
        camera: &scene.1,
        object,
        bounces: 5,
        skybox: Some(TextureID::new(&"skybox")),
    };
    let renderer = ShiftedView{
        inner: renderer,
        full_w: width,
        full_h: height,
        x: (x as f64)/(width as f64),
        y: (y as f64)/(height as f64)
    };
    let image = ArrayCollector {}.collect(renderer, texture_repo, part_width, part_height);
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
            .arg(part_height)
            .arg(part_width)
            .arg(3)
            .arg("BLOB")
            .arg(image)
            .query_async(&mut con)
            .await
            .unwrap();
        println!("Sending ACK");
        channel
            .basic_publish(
                "",
                &response,
                Default::default(),
                format!("{}#{}#{}", temp, x, y).into_bytes(),
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
    let mut textures = TextureRepository::new();
    amdl_textures::load_into(&mut textures, "../assets")?;
    texture_repo::exr::load_into(
        &mut textures,
        "../assets",
        &[(TextureID::new(&"skybox"), "skybox.exr")],
    )?;

    let mut props = PropRepository::new();
    amdl::repo::load_into(&mut props, &textures, "../assets")?;
    async_global_executor::block_on(async {
        let mut cache: LruCache<String, SceneData> = LruCache::new(5);
        let rabbitmq_client = Connection::connect(
            &amqp_addr,
            ConnectionProperties::default().with_default_executor(8),
        )
        .await?;
        let mut redis_client = redis::Client::open(redis_addr)?;


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
            let future = render(&textures, &props, &mut cache, &mut redis_client, &channel, delivery);
            if let Err(err) = future.await {
                println!("Error: {}", err);
            }
        }
        Ok(())
    })
}
