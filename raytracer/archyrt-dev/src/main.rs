use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    cameras::perspective::PerspectiveCamera,
    collector::image_collector::ImageCollector,
    intersectables::triangle::Triangle,
    loaders::{gltf::GltfLoader, Loader},
    renderers::basic_renderer::BasicRenderer,
    utilities::math::Vec3,
};

fn main() {
    let loader = GltfLoader::load("model.glb").unwrap();
    let renderer = BasicRenderer {
        object: loader.get_triangles(),
        camera: loader.get_camera(),
        lamp: Vec3::new(0.0, 0.0, 0.0),
    };
    let collector = ImageCollector {};
    let image = collector.collect(renderer, 128, 128).unwrap();
    image.save("image.png").unwrap();
}
