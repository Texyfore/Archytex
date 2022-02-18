use archyrt_core::{
    api::fragment_collector::FragmentCollector,
    collector::image_collector::ImageCollector,
    intersectables::sphere::Sphere,
    renderers::basic_renderer::BasicRenderer,
    utilities::{
        math::Vec3,
        ray::{Intersectable, Ray},
    }, textures::texture_repo::TextureRepository,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn sphere_ray_intersection(c: &mut Criterion) {
    let sphere = Sphere {
        origin: Vec3::new(0.0, 0.0, 2.0),
        radius: 1.0,
        color: Vec3::ones(),
        ..Default::default()
    };
    c.bench_function("sphere-ray", |b| {
        b.iter(|| {
            let ray = Ray {
                origin: Vec3::from_single(0.0),
                direction: Vec3::new(0.0, 0.0, 1.0),
            };
            sphere.intersect(ray).unwrap();
        })
    });
}

pub fn rendering(c: &mut Criterion) {
    c.bench_function("rendering", |b| {
        b.iter(|| {
            let repo = TextureRepository::new();
            let renderer = BasicRenderer {
                ..Default::default()
            };
            let collector = ImageCollector {};
            let image = collector.collect(renderer, repo, 10, 10).unwrap();
            black_box(image);
        })
    });
}

criterion_group!(benches, sphere_ray_intersection, rendering);
criterion_main!(benches);
