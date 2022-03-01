use std::{env, path::Path, sync::Arc};

use anyhow::{anyhow, Result};
use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    collector::raw_collector::RawCollector,
    intersectables::bvh::BVH,
    loaders::{
        ascn::{amdl_textures, ASCNLoader},
        Loader,
    },
    renderers::{solid_renderers::{albedo::AlbedoRenderer, normal::NormalRenderer}, sampling::SamplingRenderer},
    textures::texture_repo::TextureRepository, vector, utilities::math::Vec3, tonemapping::tonemap_fragment, cameras::jitter::JitterCamera,
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
    bson::{doc, oid::ObjectId, Document, DateTime},
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
    let s = String::from_utf8(delivery.data).unwrap();
    let s: Vec<&str> = s.split('#').collect();
    let user = s[1];
    let project_id = s[2];
    let s = s[0].to_string();
    let user: ObjectId = ObjectId::parse_str(user).unwrap();
    let project_id: ObjectId = ObjectId::parse_str(project_id).unwrap();
    let render_id: ObjectId = ObjectId::parse_str(&s).unwrap();
    println!("[{}] Received", render_id);
    let samples: i32 =
        redis::Cmd::get(format!("archyrt:{}:samples", s)).query(&mut redis_client).unwrap();
    let width: usize = redis::Cmd::get(format!("archyrt:{}:width", s)).query(&mut redis_client).unwrap();
    let height: usize =
        redis::Cmd::get(format!("archyrt:{}:height", s)).query(&mut redis_client).unwrap();
    //Create image storage on Redis
    let image_key = format!("archyrt:{}:image", s);
    let _: () = redis::cmd("AI.TENSORSET")
        .arg(&image_key)
        .arg("FLOAT")
        .arg(height)
        .arg(width)
        .arg(3)
        .query(&mut redis_client).unwrap();
    let payload = s.clone();
    //Put as many tasks on the queue as there are samples
    for x in 0..4 {
        for y in 0..4{
            let x = width/4*x;
            let y = height/4*y;
            futures::stream::iter(0..samples)
                .for_each(|_| async {
                    let id = Uuid::new_v4();
                    let payload = format!("{}#{}#{}#{}#{}", payload, id, response_queue, x, y);
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
        }
    }
    channel
        .basic_ack(delivery.delivery_tag, Default::default())
        .await.unwrap();
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
        .await.unwrap();
    let mut counter = 0;
    println!("[{}] Waiting for workers to finish", render_id);
    while let Some(delivery) = consumer.next().await {
        let (_, _delivery) = delivery.unwrap();


        let msg = String::from_utf8(_delivery.data).unwrap();
        let msg: Vec<&str> = msg.split("#").collect();
        let temp = msg[0];
        let x: usize = msg[1].parse().unwrap();
        let y: usize = msg[2].parse().unwrap();
        //Add image to accumulator
        let _: () = redis::cmd("AI.SCRIPTEXECUTE")
            .arg("archyrt:scripts")
            .arg("add")
            .arg("INPUTS")
            .arg(2)
            .arg(&temp)
            .arg(&image_key)
            .arg("ARGS")
            .arg(2)
            .arg(x)
            .arg(y)
            .arg("OUTPUTS")
            .arg(1)
            .arg(&image_key)
            .execute(&mut redis_client);
        //Remove temporary storage
        let _: () = redis::Cmd::del(temp).query(&mut redis_client).unwrap();
        counter += 1;
        if counter >= samples*16 {
            break;
        }

        //Update percentage
        let percentage = (counter as f32) / (samples as f32) / 16.0;
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
            .await.unwrap();
    }
    channel
        .queue_delete(response_queue, Default::default())
        .await.unwrap();
    println!("[{}] Retrieving data", render_id);
    let _: () = redis::cmd("AI.SCRIPTEXECUTE")
        .arg("archyrt:scripts")
        .arg("divide")
        .arg("INPUTS")
        .arg(1)
        .arg(&image_key)
        .arg("ARGS")
        .arg(1)
        .arg(samples)
        .arg("OUTPUTS")
        .arg(1)
        .arg(&image_key)
        .query(&mut redis_client).unwrap();
    let image: Vec<u8> = redis::cmd("AI.TENSORGET")
        .arg(&image_key)
        .arg("BLOB")
        .query(&mut redis_client).unwrap();
    let image: Vec<f32> = image
        .chunks(4)
        .map(|a| {
            let a: [u8; 4] = a.try_into().unwrap();
            f32::from_le_bytes(a)
        })
        .collect();

    //Render Albedo and Normal
    let scene: Vec<u8> =
        redis::Cmd::get(format!("archyrt:{}:scene", s)).query(&mut redis_client).unwrap();
    let scene = ASCNLoader::from_bytes(&scene).unwrap();
    let width: usize = redis::Cmd::get(format!("archyrt:{}:width", s)).query(&mut redis_client).unwrap();
    let height: usize =
        redis::Cmd::get(format!("archyrt:{}:height", s)).query(&mut redis_client).unwrap();
    let bvh = BVH::from_triangles(scene.get_triangles())
        .ok_or_else(|| anyhow!("Unable to create BVH")).unwrap();
    let camera = scene.get_camera();
    let albedo = AlbedoRenderer {
        object: &bvh,
        camera: JitterCamera::new(&camera, width, height),
    };
    let normal = NormalRenderer {
        object: &bvh,
        camera: &camera,
    };
    let collector = RawCollector {};
    println!("[{}] Rendering Albedo and Normal", render_id);
    let albedo = collector.collect(albedo, &textures, width, height);
    let normal = collector.collect(normal, &textures, width, height);
    let mut output: Vec<f32> = (0..image.len()).into_iter().map(|_| 0f32).collect();
    //Apply denoised
    println!("[{}] Applying denoiser", render_id);
    let device = oidn::Device::new();
    oidn::RayTracing::new(&device)
        .srgb(false)
        .hdr(true)
        .image_dimensions(width, height)
        .albedo_normal(&albedo, &normal)
        .clean_aux(true)
        .filter(&image, &mut output).unwrap();

    println!("[{}] Saving", render_id);
    let mut image = image::RgbImage::new(width as u32, height as u32);
    let output: Vec<Vec3> = output.chunks(3).map(|v|{
        let [r, g, b]: [f32;3] = v.try_into().unwrap();
        vector![r as f64, g as f64, b as f64]
    }).collect();
    for (x, y, color) in image.enumerate_pixels_mut() {
        let index = y as usize * width + x as usize;
        let c = output[index];
        
        let c = tonemap_fragment(c);

        let r = c.x()*255.0;
        let g = c.y()*255.0;
        let b = c.z()*255.0;
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
    image.save(path).unwrap();
    users.update_many(doc! {"_id": user}, doc!{"$set":{"projects.$[project].renders.$[render].finished": DateTime::now(), "projects.$[project].renders.$[render].status": 1.0, "projects.$[project].renders.$[render].icon": render_id.to_hex()}}, UpdateOptions::builder().array_filters(vec![doc!{"render._id": render_id}, doc!{"project._id": project_id}]).build()).await.unwrap();
    let _: () = redis::Cmd::del(&image_key).query(&mut redis_client).unwrap();
    println!("[{}] Done!", render_id);
    Ok(())
}

static TORCHSCRIPT: &str = "
def add(tensors: List[Tensor], keys: List[str], args: List[str]):
    x = int(args[0])
    y = int(args[1])
    w = tensors[0].shape[1]
    h = tensors[0].shape[0]
    t = torch.clone(tensors[1])
    t[y:y+h,x:x+w,:] += tensors[0]
    return t
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
        let mongodb_options = ClientOptions::parse(mongodb_addr).await.unwrap();
        let mongodb_client = mongodb::Client::with_options(mongodb_options).unwrap();
        let db = mongodb_client.database("archytex");
        let users = db.collection::<Document>("users");
        let rabbitmq_client = Connection::connect(
            &amqp_addr,
            ConnectionProperties::default().with_default_executor(8),
        )
        .await.unwrap();
        let mut redis_client = redis::Client::open(redis_addr).unwrap();
        let _: () = redis::cmd("AI.SCRIPTSTORE")
            .arg("archyrt:scripts")
            .arg("CPU")
            .arg("ENTRY_POINTS")
            .arg(2)
            .arg("add")
            .arg("divide")
            .arg("SOURCE")
            .arg(TORCHSCRIPT)
            .query(&mut redis_client).unwrap();

        let channel = rabbitmq_client.create_channel().await.unwrap();

        let queue = channel
            .queue_declare("archyrt:dispatch", Default::default(), Default::default())
            .await.unwrap();
        let task_queue = channel
            .queue_declare("archyrt:taskqueue", Default::default(), Default::default())
            .await.unwrap();

        let mut consumer = channel
            .basic_consume(
                queue.name().as_str(),
                "archyrt:consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await.unwrap();
        let mut textures = TextureRepository::new();
        amdl_textures::load_into(&mut textures, "../assets").unwrap();
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
                .await.unwrap();
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
