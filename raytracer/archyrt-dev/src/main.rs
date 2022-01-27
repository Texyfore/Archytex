use std::marker::PhantomData;

use archyrt_core::api::fragment_render::{FragmentContext, FragmentRender};
use archyrt_core::collector::array_collector::ArrayCollector;
use archyrt_core::intersectables::bvh::BVH;
use archyrt_core::intersectables::sphere::Sphere;
use archyrt_core::loaders::amdl::{amdl_textures, AMDLLoader};
use archyrt_core::renderers::path_tracer::{Material, PathTracer};
use archyrt_core::renderers::solid_renderers::albedo::AlbedoRenderer;
use archyrt_core::renderers::solid_renderers::normal::NormalRenderer;
use archyrt_core::textures::texture_repo::TextureRepository;
use archyrt_core::utilities::math::{Vec2, Vector};
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
use image::{Rgb, RgbImage};
use rayon::prelude::*;

struct SamplingRenderer<Renderer: FragmentRender + Sync + Send> {
    pub inner: Renderer,
    pub samples: usize,
}

impl<Renderer: FragmentRender + Sync + Send> FragmentRender for SamplingRenderer<Renderer> {
    fn render_fragment<R: TextureRepository + Sync>(
        &self,
        ctx: &FragmentContext<R>,
        pos: Vec2,
    ) -> Vec3 {
        (0..self.samples)
            .into_par_iter()
            .map(|_| self.inner.render_fragment(ctx, pos))
            .reduce(|| Vec3::default(), |a, b| a + b)
            / (self.samples as f64)
    }
}

struct TonemappingRenderer<Renderer: FragmentRender> {
    pub inner: Renderer,
}

impl<Renderer: FragmentRender> FragmentRender for TonemappingRenderer<Renderer> {
    fn render_fragment<R: TextureRepository + Sync>(
        &self,
        ctx: &FragmentContext<R>,
        pos: Vec2,
    ) -> Vec3 {
        let c = self.inner.render_fragment(ctx, pos);
        c.to_srgb()
    }
}

struct RawCollector{
    
}

impl<T: FragmentRender> FragmentCollector<T> for RawCollector{
    type Output = Vec<f32>;

    fn collect<R: TextureRepository + Sync>(
        &self,
        fragment_render: T,
        texture_repo: R,
        width: usize,
        height: usize,
    ) -> Self::Output {
        let collector = ArrayCollector{};
        collector.collect(fragment_render, texture_repo, width, height)
        .into_iter()
        .flatten()
        .map(|a| a.inner)
        .flatten()
        .map(|a| a as f32)
        .collect()
    }
}

fn main() {
    println!("Load file");
    let repo = amdl_textures::load("../assets").unwrap();
    let loader = AMDLLoader::from_path("../assets/house_inside.ascn").unwrap();
    let camera = loader.get_camera();
    let object = loader.get_triangles();
    let object = BVH::from_triangles(object).unwrap();
    let sphere_intersection = object
        .intersect(Ray {
            origin: camera.position,
            direction: camera.matrix.inner[2],
        })
        .unwrap();
    let radius = 1.0;
    let sphere = Sphere {
        origin: sphere_intersection.get_pos() + Vec3::new(0.0, radius, 0.0),
        color: Vec3::new(0.0, 1.0, 0.0),
        radius: radius,
        material: Material::Emissive { power: 10.0 },
    };
    let object = object.union(sphere);
    println!("Render");
    let renderer = PathTracer {
        object: &object,
        camera: &camera,
        bounces: 5,
    };
    let renderer = SamplingRenderer {
        inner: renderer,
        samples: 5,
    };
    let albedo = AlbedoRenderer{object: &object, camera: &camera};
    let normal = NormalRenderer{object: &object, camera: &camera};
    let collector = RawCollector {};
    let w = 1920;
    let h = 1080;
    println!("Rendering image");
    let rt_image = collector
        .collect(&renderer, &repo, w, h);
    let albedo_image = collector
        .collect(&albedo, &repo, w, h);
    let normal_image = collector
        .collect(&normal, &repo, w, h);
    println!("Denoising");
    let mut output: Vec<f32> = (0..rt_image.len()).into_iter().map(|_| 0f32).collect();
    let device = oidn::Device::new();
    oidn::RayTracing::new(&device)
        .srgb(false)
        .image_dimensions(w, h)
        .albedo_normal(&albedo_image, &normal_image)
        .filter(&rt_image, &mut output)
        .unwrap();
    let mut image = RgbImage::new(w as u32, h as u32);
    for (x, y, color) in image.enumerate_pixels_mut() {
        let index = (y as usize * w + x as usize) * 3;
        let r = output[index + 0].powf(1.0 / 2.2) * 255.0;
        let g = output[index + 1].powf(1.0 / 2.2) * 255.0;
        let b = output[index + 2].powf(1.0 / 2.2) * 255.0;
        let r = r.clamp(0.0, 255.0);
        let g = g.clamp(0.0, 255.0);
        let b = b.clamp(0.0, 255.0);
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        *color = Rgb([r, g, b]);
    }
    image.save("image.png").unwrap();
}
