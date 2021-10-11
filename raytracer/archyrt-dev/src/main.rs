use archyrt_core::{
    api::fragment_collector::FragmentCollector, cameras::perspective::PerspectiveCamera,
    collector::image_collector::ImageCollector, intersectables::surface::Surface,
    renderers::basic_renderer::BasicRenderer, utilities::math::Vec3,
};

fn main() {
    let renderer = BasicRenderer {
        object: Surface {
            distance: -4.0,
            normal: Vec3::new(0.0, 1.0, 0.0),
            color: Vec3::from_single(1.0),
        },
        camera: PerspectiveCamera::new(Vec3::default(), Vec3::new(0.0, 0.0, 1.0), 1.0),
        lamp: Vec3::new(0.0, 1.0, 40.0),
    };
    let collector = ImageCollector {};
    let image = collector.collect(renderer, 1024, 1024).unwrap();
    image.save("image.png").unwrap();
}
