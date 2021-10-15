use crate::utilities::{
    math::Vec3,
    ray::{Intersectable, Intersection, IntersectionBuilder, Ray},
};

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

fn min_vec(a: Vec3, b: Vec3) -> Vec3 {
    let mut o = a;
    for (a, b) in o.inner.iter_mut().zip(b.inner) {
        *a = a.min(b);
    }
    o
}
fn max_vec(a: Vec3, b: Vec3) -> Vec3 {
    let mut o = a;
    for (a, b) in o.inner.iter_mut().zip(b.inner) {
        *a = a.max(b);
    }
    o
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        let min = min_vec(a, b);
        let max = max_vec(a, b);

        Self { min, max }
    }
}

impl AABB {
    pub fn intersect(&self, ray: Ray) -> Option<f64> {
        //https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection
        // Why?
        let (tmin, tmax) = if ray.direction.x() < 0.0 {
            (self.max.x(), self.min.x())
        } else {
            (self.min.x(), self.max.x())
        };
        let tmin = (tmin - ray.origin.x()) / ray.direction.x();
        let tmax = (tmax - ray.origin.x()) / ray.direction.x();

        let (tymin, tymax) = if ray.direction.y() < 0.0 {
            (self.max.y(), self.min.y())
        } else {
            (self.min.y(), self.max.y())
        };
        let tymin = (tymin - ray.origin.y()) / ray.direction.y();
        let tymax = (tymax - ray.origin.y()) / ray.direction.y();

        if tmin > tymax || tymin > tmax {
            return None;
        }
        let tmin = tmin.max(tymin);
        let tmax = tmax.min(tymax);

        let (tzmin, tzmax) = if ray.direction.z() < 0.0 {
            (self.max.z(), self.min.z())
        } else {
            (self.min.z(), self.max.z())
        };
        let tzmin = (tzmin - ray.origin.z()) / ray.direction.z();
        let tzmax = (tzmax - ray.origin.z()) / ray.direction.z();

        if tmin > tzmax || tzmin > tmax {
            return None;
        }
        let tmin = tmin.max(tzmin);
        let tmax = tmax.min(tzmax);
        Some(tmin)
    }
}

pub struct AABBRay {
    pub aabb: AABB,
    pub color: Vec3,
}

impl Intersectable for AABBRay {
    fn intersect(&self, ray: Ray) -> Option<Intersection> {
        let t = self.aabb.intersect(ray)?;
        let o = (self.aabb.min + self.aabb.max) / 2.0;
        let p = ray.direction * t + ray.origin;
        let d = p - o;
        let (i, _) = d
            .inner
            .iter()
            .map(|v| f64::abs(*v))
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        let normal = if d[i] > 0.0 {
            let mut v = Vec3::default();
            v[i] = 1.0;
            v
        } else {
            let mut v = Vec3::default();
            v[i] = -1.0;
            v
        };
        Some(
            IntersectionBuilder {
                ray,
                distance: Some(t),
                pos: Some(p),
                normal,
                color: self.color,
                ..Default::default()
            }
            .build(),
        )
    }
}
