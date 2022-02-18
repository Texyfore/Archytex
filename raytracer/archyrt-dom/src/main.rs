use std::{env, path::Path, sync::Arc};

use anyhow::{anyhow, Result};
use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    collector::raw_collector::RawCollector,
    intersectables::bvh::BVH,
    loaders::{
        amdl::{amdl_textures, AMDLLoader},
        Loader,
    },
    renderers::solid_renderers::{albedo::AlbedoRenderer, normal::NormalRenderer},
    textures::texture_repo::TextureRepository,
};
use dotenv::dotenv;

use futures_util::stream::StreamExt;
use image::Rgb;
use lapin::{
    message::Delivery,
    options::{BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
    Channel, Connection, ConnectionProperties, Queue,
};
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{ClientOptions, UpdateOptions},
    Collection,
};
use uuid::Uuid;

async fn handle_job(
    users: Collection<Document>,
    mut redis_client: redis::Client,
    channel: Channel,
    delivery: Delivery,
    response_queue: Queue,
    task_queue: Queue,
    textures: Arc<TextureRepository>,
) -> Result<()> {
    let response_queue = response_queue.name().as_str();
    let task_queue = task_queue.name().as_str();
    let s = String::from_utf8(delivery.data)?;
    let s: Vec<&str> = s.split('#').collect();
    let user = s[1];
    let project_id = s[2];
    let s = s[0].to_string();
    let user: ObjectId = ObjectId::parse_str(user)?;
    let project_id: ObjectId = ObjectId::parse_str(project_id)?;
    let render_id: ObjectId = ObjectId::parse_str(&s)?;
    println!("[{}] Received", render_id);
    let samples: i32 =
        redis::Cmd::get(format!("archyrt:{}:samples", s)).query(&mut redis_client)?;
    let width: usize = redis::Cmd::get(format!("archyrt:{}:width", s)).query(&mut redis_client)?;
    let height: usize =
        redis::Cmd::get(format!("archyrt:{}:height", s)).query(&mut redis_client)?;
    //Create image storage on Redis
    let image_key = format!("archyrt:{}:image", s);
    redis::cmd("AI.TENSORSET")
        .arg(&image_key)
        .arg("FLOAT")
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
    println!("[{}] Waiting for workers to finish", render_id);
    while let Some(delivery) = consumer.next().await {
        let (_, _delivery) = delivery.unwrap();
        counter += 1;
        if counter >= samples {
            break;
        }

        //Update percentage
        let percentage = (counter as f32) / (samples as f32);
        users
            .update_many(
                doc! {"_id": user},
                doc! {"$set":{"projects.$[project].renders.$[render].status": percentage}},
                UpdateOptions::builder()
                    .array_filters(vec![
                        doc! {"render._id": render_id},
                        doc! {"project._id": project_id},
                    ])
                    .build(),
            )
            .await?;
    }
    channel
        .queue_delete(response_queue, Default::default())
        .await?;
    println!("[{}] Retrieving data", render_id);
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
    let image: Vec<f32> = image
        .chunks(4)
        .map(|a| {
            let a: [u8; 4] = a.try_into().unwrap();
            f32::from_le_bytes(a)
        })
        .collect();

    println!("[{}] Applying denoiser", render_id);
    //Render Albedo and Normal
    let scene: Vec<u8> =
        redis::Cmd::get(format!("archyrt:{}:scene", s)).query(&mut redis_client)?;
    let scene = AMDLLoader::from_bytes(&scene)?;
    let width: usize = redis::Cmd::get(format!("archyrt:{}:width", s)).query(&mut redis_client)?;
    let height: usize =
        redis::Cmd::get(format!("archyrt:{}:height", s)).query(&mut redis_client)?;
    let bvh = BVH::from_triangles(scene.get_triangles())
        .ok_or_else(|| anyhow!("Unable to create BVH"))?;
    let camera = scene.get_camera();
    let albedo = AlbedoRenderer {
        object: &bvh,
        camera: &camera,
    };
    let normal = NormalRenderer {
        object: &bvh,
        camera: &camera,
    };
    let collector = RawCollector {};
    let albedo = collector.collect(albedo, &textures, width, height);
    let normal = collector.collect(normal, &textures, width, height);
    let mut output: Vec<f32> = (0..image.len()).into_iter().map(|_| 0f32).collect();
    let device = oidn::Device::new();
    oidn::RayTracing::new(&device)
        .srgb(false)
        .hdr(true)
        .image_dimensions(width, height)
        .albedo_normal(&albedo, &normal)
        .clean_aux(true)
        .filter(&image, &mut output)
        .unwrap();

    println!("[{}] Saving", render_id);
    let mut image = image::RgbImage::new(width as u32, height as u32);
    for (x, y, color) in image.enumerate_pixels_mut() {
        let index = (y as usize * width + x as usize) * 3;
        let r = output[index + 0].powf(1.0 / 2.2) * 255.0;
        let g = output[index + 1].powf(1.0 / 2.2) * 255.0;
        let b = output[index + 2].powf(1.0 / 2.2) * 255.0;
        let r = r.clamp(0.0, 255.0);
        let g = g.clamp(0.0, 255.0);
        let b = b.clamp(0.0, 255.0);
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        *color = Rgb([r, g, b]);
    }
    let path = Path::new(&env::var("IMAGES").unwrap())
        .join(s)
        .with_extension("png");
    image.save(path)?;
    users.update_many(doc! {"_id": user}, doc!{"$set":{"projects.$[project].renders.$[render].status": 1.0, "projects.$[project].renders.$[render].icon": render_id.to_hex()}}, UpdateOptions::builder().array_filters(vec![doc!{"render._id": render_id}, doc!{"project._id": project_id}]).build()).await?;
    redis::Cmd::del(&image_key).query(&mut redis_client)?;
    println!("[{}] Done!", render_id);
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
    let mongodb_addr = env::var("MONGODB_ADDR").unwrap();
    env::var("IMAGES").unwrap();
    async_global_executor::block_on(async {
        let mongodb_options = ClientOptions::parse(mongodb_addr).await?;
        let mongodb_client = mongodb::Client::with_options(mongodb_options)?;
        let db = mongodb_client.database("archytex");
        let users = db.collection::<Document>("users");
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
        let mut textures = TextureRepository::new();
        amdl_textures::load_into(&mut textures, "../assets")?;
        let textures = Arc::new(textures);

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
                users.clone(),
                redis_client.clone(),
                channel.clone(),
                delivery,
                response_queue,
                task_queue.clone(),
                textures.clone(),
            ))
            .detach();
        }

        Ok(())
    })
}
