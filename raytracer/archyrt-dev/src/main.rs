use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    cameras::perspective::PerspectiveCamera,
    collector::image_collector::ImageCollector,
    intersectables::triangle::Triangle,
    loaders::{Loader},
    renderers::basic_renderer::BasicRenderer,
    utilities::math::Vec3,
};
use archyrt_core::loaders::amdl::AMDLLoader;

fn main() {
    println!("Load file");
    let pos = Vec3::new(7.0, 1.0, 1.0)*2.0;
    let target = Vec3::new(0.0, 0.0, 0.0);
    let loader = AMDLLoader::from_path("test.amdl").unwrap();
    let camera = PerspectiveCamera::new(
        pos,
        (target-pos).normalized(), 0.595877);
    println!("Render");
    let renderer = BasicRenderer {
        object: loader.get_triangles(),
        camera: camera,
        lamp: pos,
    };
    let collector = ImageCollector {};
    let image = collector.collect(renderer, 128, 128).unwrap();
    image.save("image.png").unwrap();
}
