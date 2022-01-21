use std::marker::PhantomData;

use archyrt_core::api::fragment_render::{FragmentRender, FragmentContext};
use archyrt_core::collector::array_collector::ArrayCollector;
use archyrt_core::intersectables::bvh::BVH;
use archyrt_core::intersectables::sphere::Sphere;
use archyrt_core::loaders::amdl::{amdl_textures, AMDLLoader};
use archyrt_core::renderers::path_tracer::{PathTracer, Material};
use archyrt_core::textures::texture_repo::TextureRepository;
use archyrt_core::utilities::math::{Vector, Vec2};
use archyrt_core::utilities::ray::{Intersectable, Ray};
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

struct TonemappingRenderer<Renderer: FragmentRender>{pub inner: Renderer}

impl<Renderer: FragmentRender> FragmentRender for TonemappingRenderer<Renderer>{
    fn render_fragment<R: TextureRepository>(&self, ctx: &FragmentContext<R>, pos: Vec2) -> Vec3 {
        let c = self.inner.render_fragment(ctx, pos);
        c.to_srgb()
    }
}

fn main() {
    println!("Load file");
    let repo = amdl_textures::load("../assets").unwrap();
    let loader = AMDLLoader::from_path("../assets/house_inside.ascn").unwrap();
    let camera = loader.get_camera();
    let object = loader.get_triangles();
    let object = BVH::from_triangles(object).unwrap();
    let sphere_intersection = object.intersect(Ray{
        origin: camera.position,
        direction: camera.matrix.inner[2]
    }).unwrap();
    let radius = 1.0;
    let sphere = Sphere{
        origin: sphere_intersection.get_pos() + Vec3::new(0.0, radius, 0.0),
        color: Vec3::new(0.0, 1.0, 0.0),
        radius: radius,
        material: Material::Emissive{power: 1.0}
    };
    let object = object.union(sphere);
    println!("Render");
    let renderer = PathTracer {
        object,
        camera,
        bounces: 5,
    };
    let renderer = SamplingRenderer{
        inner: renderer,
        samples: 20
    };
    let renderer = TonemappingRenderer{
        inner: renderer
    };
    let collector = ImageCollector {};
    let image = collector.collect(&renderer, &repo, 1920/2, 1080/2).unwrap();
    image.save("image.png").unwrap();
}
