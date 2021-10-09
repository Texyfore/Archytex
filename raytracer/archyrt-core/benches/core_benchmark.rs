use archyrt_core::{
    intersectables::sphere::Sphere,
    utilities::{
        math::Vec3,
        ray::{Intersectable, Ray},
    },
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn sphere_ray_intersection(c: &mut Criterion) {
    let sphere = Sphere {
        origin: Vec3::new(0.0, 0.0, 2.0),
        radius: 1.0,
        color: Vec3::ones(),
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

criterion_group!(benches, sphere_ray_intersection);
criterion_main!(benches);
