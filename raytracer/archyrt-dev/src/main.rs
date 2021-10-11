use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    cameras::perspective::PerspectiveCamera,
    collector::image_collector::ImageCollector,
    intersectables::triangle::Triangle,
    renderers::basic_renderer::BasicRenderer,
    utilities::math::Vec3,
};

fn main() {
    let renderer = BasicRenderer {
        object: Triangle::default(),
        camera: PerspectiveCamera::new(Vec3::default(), Vec3::new(0.0, 0.0, 1.0), 1.0),
        lamp: Vec3::new(0.0, 0.0, 0.0),
    };
    let collector = ImageCollector {};
    let image = collector.collect(renderer, 1024, 1024).unwrap();
    image.save("image.png").unwrap();
}
