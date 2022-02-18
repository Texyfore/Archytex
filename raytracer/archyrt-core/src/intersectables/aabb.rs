use crate::utilities::math::Axis3;
use crate::vector;
use crate::{
    textures::color_provider::SolidColor,
    utilities::{
        math::Vec3,
        ray::{Intersectable, Intersection, IntersectionBuilder, Ray},
    },
};

#[derive(Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        let min = a.min(b);
        let max = a.max(b);

        Self { min, max }
    }
    pub fn union(self, rhs: Self) -> AABB {
        AABB {
            min: self.min.min(rhs.min),
            max: self.max.max(rhs.max),
        }
    }
    pub fn max_axis(self) -> Axis3 {
        (self.max - self.min).max_axis()
    }
}

impl AABB {
    pub fn intersect(&self, ray: Ray) -> Option<f64> {
        let invdir = vector![
            1. / ray.direction[0],
            1. / ray.direction[1],
            1. / ray.direction[2]
        ];
        let (mut tmin, mut tmax) = if invdir.x() >= 0.0 {
            let tmin = (self.min.x() - ray.origin.x()) * invdir.x();
            let tmax = (self.max.x() - ray.origin.x()) * invdir.x();
            (tmin, tmax)
        } else {
            let tmin = (self.max.x() - ray.origin.x()) * invdir.x();
            let tmax = (self.min.x() - ray.origin.x()) * invdir.x();
            (tmin, tmax)
        };
        let (tymin, tymax) = if invdir.y() >= 0.0 {
            let tymin = (self.min.y() - ray.origin.y()) * invdir.y();
            let tymax = (self.max.y() - ray.origin.y()) * invdir.y();
            (tymin, tymax)
        } else {
            let tymin = (self.max.y() - ray.origin.y()) * invdir.y();
            let tymax = (self.min.y() - ray.origin.y()) * invdir.y();
            (tymin, tymax)
        };

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }
        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let (tzmin, tzmax) = if invdir.z() >= 0.0 {
            let tzmin = (self.min.z() - ray.origin.z()) * invdir.z();
            let tzmax = (self.max.z() - ray.origin.z()) * invdir.z();
            (tzmin, tzmax)
        } else {
            let tzmin = (self.max.z() - ray.origin.z()) * invdir.z();
            let tzmax = (self.min.z() - ray.origin.z()) * invdir.z();
            (tzmin, tzmax)
        };

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }
        if tzmin > tmin {
            tmin = tzmin;
        }
        if tzmax < tmax {
            tmax = tzmax;
        }

        let t = if tmin < 0.0 { tmax } else { tmin };
        if tmax < 0.0 {
            return None;
        }
        Some(t)
    }
}

pub struct AABBRay {
    pub aabb: AABB,
    pub color: Vec3,
}

impl Intersectable for AABBRay {
    type C = SolidColor;
    fn intersect(&self, ray: Ray) -> Option<Intersection<Self::C>> {
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
            let mut v = Vec3::from_single(0.0);
            v[i] = 1.0;
            v
        } else {
            let mut v = Vec3::from_single(0.0);
            v[i] = -1.0;
            v
        };
        Some(
            IntersectionBuilder {
                ray,
                distance: Some(t),
                pos: Some(p),
                normal,
                ..Default::default()
            }
            .build(),
        )
    }
}
