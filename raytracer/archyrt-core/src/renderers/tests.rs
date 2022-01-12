mod basic_renderer {
    use crate::{
        api::fragment_collector::FragmentCollector, collector::image_collector::ImageCollector,
        renderers::basic_renderer::BasicRenderer, textures::texture_repo::DummyTextureRepository,
    };

    #[test]
    fn compare_default() {
        let repo = DummyTextureRepository {};
        let scene = BasicRenderer::default();
        let collector = ImageCollector {};
        let image = collector.collect(scene, repo, 128, 128).unwrap();
        let reference = image::open("tests/compare_default.png").unwrap();
        let reference = reference.as_rgb8().unwrap();
        assert_eq!(*reference, image);
    }
}
