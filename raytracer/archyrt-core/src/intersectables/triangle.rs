use crate::renderers::path_tracer::Material;
use crate::textures::color_provider::ColorProvider;
use crate::textures::samplers::nearest::NearestSampler;
use crate::textures::samplers::TextureSampler;
use crate::textures::texture_repo::TextureRepository;
use crate::textures::TextureID;
use crate::utilities::math::{Axis3, Vec2};
use crate::vector;
use crate::{
    matrix,
    utilities::{
        math::{Matrix3x3, Vec3},
        ray::{Intersectable, Intersection, IntersectionBuilder, Ray},
    },
};
use std::cmp::Ordering;

use super::aabb::AABB;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub normal: Vec3,
    pub uv: [Vec2; 3],
    pub texture: TextureID,
    pub material: Material
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3], uv: [Vec2; 3], texture: TextureID, material: Material) -> Self {
        let [a, b, c] = vertices;
        let normal = (b - a).cross(c - a).normalized();
        Self {
            a,
            b,
            c,
            normal,
            uv,
            texture,
            material,
        }
    }
    pub fn bounds(&self) -> AABB {
        let min = self.a.min(self.b).min(self.c);
        let max = self.a.max(self.b).max(self.c);
        AABB { min, max }
    }

    pub fn centroid(&self) -> Vec3 {
        (self.a + self.b + self.c) / 3.0
    }

    pub fn side(&self, a: Axis3, divider: f64) -> Ordering {
        let o = self.a.get(a) >= divider;
        for p in [self.b, self.c].iter() {
            if (p.get(a) >= divider) != o {
                return Ordering::Equal;
            }
        }
        if o {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        let a = Vec3::new(0.0, 0.0, 3.0);
        let b = Vec3::new(1.0, -1.0, 3.0);
        let c = Vec3::new(-1.0, -1.0, 3.0);
        let uv = [vector![0.0, 0.0], vector![0.0, 1.0], vector![1.0, 0.0]];
        Self::new([a, b, c], uv, TextureID(1), Material::Diffuse)
    }
}

#[derive(Default)]
pub struct TriangleColor {
    pub uv: [Vec2; 3],
    pub barycentric: Vec3,
    pub texture: TextureID,
    pub material: Material
}

impl ColorProvider for TriangleColor {
    fn get_color<R: TextureRepository>(&self, repo: &R) -> Vec3 {
        let sampler = NearestSampler {};
        let coords = self.uv[1] * self.barycentric[0]
            + self.uv[2] * self.barycentric[1]
            + self.uv[0] * self.barycentric[2];
        sampler.sample(repo, self.texture, coords)
    }

    fn get_material(&self) -> Material {
        self.material
    }
}

impl Intersectable for Triangle {
    type C = TriangleColor;
    fn intersect(&self, ray: Ray) -> Option<Intersection<Self::C>> {
        //Based on https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/moller-trumbore-ray-triangle-intersection
        //Backface culling
        if self.normal.dot(ray.direction) > 0.0 {
            return None;
        }
        let mat: Matrix3x3 = matrix!(-ray.direction, self.b - self.a, self.c - self.a);
        let v = ray.origin - self.a;
        let solution = mat.cramer(v)?;
        let [t, u, v] = solution.inner;
        if t < 0.0 || u < 0.0 || v < 0.0 || u + v > 1.0 {
            return None;
        }
        Some(
            IntersectionBuilder {
                ray,
                distance: Some(t),
                normal: self.normal,
                color_provider: TriangleColor {
                    uv: self.uv,
                    barycentric: Vec3::new(u, v, 1.0 - u - v),
                    texture: self.texture,
                    material: self.material
                },
                ..Default::default()
            }
            .build(),
        )
    }
}
