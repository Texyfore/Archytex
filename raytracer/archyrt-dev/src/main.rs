use std::f64::consts::PI;

use archyrt_core::api::camera::Camera;
use archyrt_core::api::fragment_render::{FragmentContext, FragmentRender};

use archyrt_core::cameras::jitter::JitterCamera;
use archyrt_core::cameras::perspective::PerspectiveCamera;
use archyrt_core::collector::image_collector::ImageCollector;
use archyrt_core::collector::raw_collector::RawCollector;
use archyrt_core::intersectables::apply_matrix::ApplyMatrix;
use archyrt_core::intersectables::bvh::BVH;
use archyrt_core::intersectables::sphere::Sphere;
use archyrt_core::intersectables::transform::Transform;
use archyrt_core::loaders::amdl::{AMDLLoader, self};
use archyrt_core::loaders::amdl::repo::{PropRepository, PropType};
use archyrt_core::loaders::ascn::{amdl_textures, ASCNLoader};
use archyrt_core::renderers::basic_renderer::BasicRenderer;
use archyrt_core::renderers::path_tracer::{Material, PathTracer};
use archyrt_core::renderers::sampling::SamplingRenderer;
use archyrt_core::renderers::solid_renderers::albedo::AlbedoRenderer;
use archyrt_core::renderers::solid_renderers::normal::NormalRenderer;
use archyrt_core::textures::texture_repo::{self, TextureRepository};
use archyrt_core::tonemapping::tonemap_fragment;
use archyrt_core::utilities::math::{Vec2, Vector, Matrix3x3};
use archyrt_core::utilities::ray::{Intersectable, Ray};
use archyrt_core::vector;
use archyrt_core::{
    api::fragment_collector::FragmentCollector, loaders::Loader, textures::TextureID,
    utilities::math::Vec3,
};
use image::{Rgb, RgbImage};
use rayon::prelude::*;

pub struct ParallelSamplingRenderer<Renderer: FragmentRender + Sync + Send> {
    pub inner: Renderer,
    pub samples: usize,
}

impl<Renderer: FragmentRender + Sync + Send> FragmentRender for ParallelSamplingRenderer<Renderer> {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        (0..self.samples)
            .into_par_iter()
            .map(|_| self.inner.render_fragment(ctx, pos))
            .reduce(Vec3::default, |a, b| a + b)
            / (self.samples as f64)
    }
}

fn render_pathtraced<O: Intersectable+Sync, C: Camera+Sync>(object: O, camera: C, mut repo: TextureRepository, w: usize, h: usize) -> image::ImageBuffer<Rgb<u8>, Vec<u8>> {
    let aa_camera = JitterCamera::new(&camera, w, h); //Camera used for anti-aliasing
    let skybox_id = TextureID::new(&"skybox");
    texture_repo::exr::load_into(&mut repo, "../assets", &[(skybox_id, "skybox.exr")]).unwrap();
    let skybox = Some(skybox_id);
    //Set renderers up
    let pathtracer = PathTracer {
        skybox,
        object: &object,
        camera: &aa_camera,
        bounces: 5,
    };
    let pathtracer = ParallelSamplingRenderer {
        inner: pathtracer,
        samples: 5,
    };
    //Albedo and Normal renderers are required by OIDN
    let albedo = AlbedoRenderer {
        object: &object,
        camera: &aa_camera,
    };
    //Make sure albedo is anti-aliased
    let albedo = ParallelSamplingRenderer {
        inner: albedo,
        samples: 5,
    };
    let normal = NormalRenderer {
        object: &object,
        camera: &camera,
    };

    //Collect images to arrays
    println!("Rendering image");
    let collector = RawCollector {};
    let pathtracer_image = collector.collect(&pathtracer, &repo, w, h);
    let albedo_image = collector.collect(&albedo, &repo, w, h);
    let normal_image = collector.collect(&normal, &repo, w, h);

    //Using OIDN for denoising
    println!("Denoising");
    let mut output: Vec<f32> = (0..pathtracer_image.len())
        .into_iter()
        .map(|_| 0f32)
        .collect();
    let device = oidn::Device::new();
    oidn::RayTracing::new(&device)
        .srgb(false)
        .hdr(true)
        .image_dimensions(w, h)
        .albedo_normal(&albedo_image, &normal_image)
        .clean_aux(true)
        .filter(&pathtracer_image, &mut output)
        .unwrap();
    //Collect OIDN image
    let mut image = RgbImage::new(w as u32, h as u32);

    let output: Vec<Vec3> = output
        .chunks(3)
        .map(|v| {
            let [r, g, b]: [f32; 3] = v.try_into().unwrap();
            vector![r as f64, g as f64, b as f64]
        })
        .collect();
    for (x, y, color) in image.enumerate_pixels_mut() {
        let index = y as usize * w + x as usize;
        let c = output[index];

        let c = tonemap_fragment(c);

        let c = c * 255.;
        let r = c.x();
        let g = c.y();
        let b = c.z();
        let r = r.clamp(0.0, 255.0);
        let g = g.clamp(0.0, 255.0);
        let b = b.clamp(0.0, 255.0);
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        *color = Rgb([r, g, b]);
    }
    image
}

fn render_albedo<O: Intersectable, C: Camera>(object: O, camera: C, mut repo: TextureRepository, w: usize, h: usize) -> image::ImageBuffer<Rgb<u8>, Vec<u8>>{
    let renderer = BasicRenderer{
        camera,
        object,
        lamp: vector![3.0, 2.0, 0.0]
    };
    let collector = RawCollector{};
    let mut image = RgbImage::new(w as u32, h as u32);
    let output = collector.collect(renderer, &repo, w, h);

    let output: Vec<Vec3> = output
        .chunks(3)
        .map(|v| {
            let [r, g, b]: [f32; 3] = v.try_into().unwrap();
            vector![r as f64, g as f64, b as f64]
        })
        .collect();
    for (x, y, color) in image.enumerate_pixels_mut() {
        let index = y as usize * w + x as usize;
        let c = output[index];

        let c = tonemap_fragment(c);

        let c = c * 255.;
        let r = c.x();
        let g = c.y();
        let b = c.z();
        let r = r.clamp(0.0, 255.0);
        let g = g.clamp(0.0, 255.0);
        let b = b.clamp(0.0, 255.0);
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        *color = Rgb([r, g, b]);
    }
    image
}

fn main() {
    let w = 512;
    let h = 512;

    //Loading textures and skybox
    println!("Load file");
    let mut textures = TextureRepository::new();
    amdl_textures::load_into(&mut textures, "../assets").unwrap();

    let mut props = PropRepository::new();
    amdl::repo::load_into(&mut props,&textures,  "../assets").unwrap();

    //Load model
    let loader = ASCNLoader::from_path("../assets/proprot.ascn").unwrap();
    let camera = loader.get_camera();
    let object = loader.get_triangles();
    let props = props.fulfill_all(loader.get_prop_requests()).unwrap();
    let object = object.union(props);

    println!("Render");
    let image = render_pathtraced(object, camera, textures, w, h);
    //let image = render_albedo(object, camera, textures, w, h);
    image.save("image.png").unwrap();
}
