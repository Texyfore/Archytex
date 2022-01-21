use std::marker::PhantomData;

use archyrt_core::api::fragment_render::{FragmentRender, FragmentContext};
use archyrt_core::collector::array_collector::ArrayCollector;
use archyrt_core::intersectables::bvh::BVH;
use archyrt_core::loaders::amdl::{amdl_textures, AMDLLoader};
use archyrt_core::renderers::path_tracer::PathTracer;
use archyrt_core::textures::texture_repo::TextureRepository;
use archyrt_core::utilities::math::{Vector, Vec2};
use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    cameras::perspective::PerspectiveCamera,
    collector::image_collector::ImageCollector,
    loaders::Loader,
    renderers::basic_renderer::BasicRenderer,
    textures::{texture_repo::png::PngTextureRepo, TextureID},
    utilities::math::Vec3,
};

struct SamplingRenderer<Renderer: FragmentRender>{
    pub inner: Renderer,
    pub samples: usize,
}

impl<Renderer: FragmentRender> FragmentRender for SamplingRenderer<Renderer>{
    fn render_fragment<R: TextureRepository>(&self, ctx: &FragmentContext<R>, pos: Vec2) -> Vec3 {
        (0..self.samples).map(|_|self.inner.render_fragment(ctx, pos)).fold(Vec3::default(), |a, b|a+b) / (self.samples as f64)
    }
}

fn main() {
    println!("Load file");
    let pos = Vec3::new(7.0, 1.0, 1.0) * 2.0;
    let repo = amdl_textures::load("../assets").unwrap();
    let loader = AMDLLoader::from_path("../assets/house_inside.ascn").unwrap();
    let object = loader.get_triangles();
    let object = BVH::from_triangles(object).unwrap();
    let camera = loader.get_camera();
    println!("Render");
    let renderer = PathTracer {
        object,
        camera,
        bounces: 5,
    };
    let renderer = SamplingRenderer{
        inner: renderer,
        samples: 10
    };
    let collector = ImageCollector {};
    let image = collector.collect(&renderer, &repo, 1920/2, 1080/2).unwrap();
    image.save("image.png").unwrap();
}
