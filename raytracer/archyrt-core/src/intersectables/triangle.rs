use std::cmp::Ordering;
use crate::{
    matrix,
    utilities::{
        math::{Matrix3x3, Vec3},
        ray::{Intersectable, Intersection, IntersectionBuilder, Ray},
    },
};
use crate::utilities::math::Axis3;

use super::aabb::AABB;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3], color: Vec3) -> Self {
        let [a, b, c] = vertices;
        let normal = (b - a).cross(c - a).normalized();
        Self {
            a,
            b,
            c,
            color,
            normal,
        }
    }
    pub fn bounds(&self) -> AABB {
        let min = self.a.min(self.b).min(self.c);
        let max = self.a.max(self.b).max(self.c);
        AABB { min, max }
    }

    pub fn centroid(&self) -> Vec3{
        (self.a+self.b+self.c)/3.0
    }

    pub fn side(&self, a: Axis3, divider: f64) -> Ordering{
        let o = self.a.get(a) >= divider;
        for p in [self.b, self.c].iter(){
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
        Self::new([a, b, c], Vec3::from_single(1.0))
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
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
                color: self.color,
                ..Default::default()
            }
            .build(),
        )
    }
}
