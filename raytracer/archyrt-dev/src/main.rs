use std::collections::HashMap;

use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    cameras::perspective::PerspectiveCamera,
    collector::image_collector::ImageCollector,
    loaders::{Loader},
    renderers::basic_renderer::BasicRenderer,
    utilities::math::Vec3, textures::{texture_repo::png::PngTextureRepo, TextureID},
};
use archyrt_core::intersectables::bvh::BVH;
use archyrt_core::loaders::amdl::AMDLLoader;

fn main() {
    println!("Load file");
    let pos = Vec3::new(7.0, 1.0, 1.0)*2.0;
    let target = Vec3::new(0.0, 0.0, 0.0);
    let repo = PngTextureRepo::new("../../frontend/public/assets", &[(TextureID(1), "nodraw.png"), (TextureID(2), "amogus.png")]).unwrap();
    let loader = AMDLLoader::from_path("test.amdl").unwrap();
    let camera = PerspectiveCamera::new(
        pos,
        (target-pos).normalized(), 0.595877);
    let object = loader.get_triangles();
    let object = BVH::from_triangles(object).unwrap();
    //let camera = loader.get_camera()
    println!("Render");
    let renderer = BasicRenderer {
        object,
        camera,
        lamp: pos,
    };
    let collector = ImageCollector {};
    let image = collector.collect(renderer,repo, 1920/2, 1080/2).unwrap();
    image.save("image.png").unwrap();
}
