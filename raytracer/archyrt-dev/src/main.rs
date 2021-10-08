use archyrt_core::{
    api::fragment_collector::FragmentCollector, collector::image_collector::ImageCollector,
    renderers::basic_renderer::BasicRenderer,
};

fn main() {
    let renderer = BasicRenderer {
        ..Default::default()
    };
    let collector = ImageCollector {};
    let image = collector.collect(renderer, 1024, 1024).unwrap();
    image.save("image.png").unwrap();
}
