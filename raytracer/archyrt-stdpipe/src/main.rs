use std::{env, io::Read};

use archyrt_core::{loaders::{amdl::{amdl_textures, AMDLLoader}, Loader}, intersectables::{bvh::BVH, aabb::AABBRay}, renderers::basic_renderer::BasicRenderer, collector::{array_collector::ArrayCollector, image_collector::ImageCollector}, utilities::math::Vec3, api::fragment_collector::FragmentCollector, cameras::perspective::PerspectiveCamera};
use image::{ColorType, EncodableLayout};


fn main() {
    let args: Vec<String> = env::args().collect();
    let width: usize = args[2].parse().unwrap();
    let height: usize = args[3].parse().unwrap();
    let mut data = Vec::new();
    std::io::stdin().read_to_end(&mut data).unwrap();
    let repo = amdl_textures::load(args[1].as_str()).unwrap();
    let scene = AMDLLoader::from_bytes(&data).unwrap();
    let bvh = BVH::from_triangles(scene.get_triangles()).unwrap();
    let object = bvh;
    let camera = scene.get_camera();
    let renderer = BasicRenderer{
        camera,
        object,
        lamp: Vec3::new(5.0, 5.0, 5.0)
    };
    
    let image = ImageCollector{}.collect(renderer, repo, width, height).unwrap();
    let encoder = image::codecs::png::PngEncoder::new(std::io::stdout());
    encoder.encode(image.as_bytes(), image.width(), image.height(), ColorType::Rgb8).unwrap();

}
