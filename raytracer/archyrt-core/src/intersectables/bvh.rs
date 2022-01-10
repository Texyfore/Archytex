use std::cmp::Ordering;
use crate::intersectables::aabb::AABB;
use crate::intersectables::triangle::Triangle;
use crate::textures::color_provider::SolidColor;
use crate::utilities::math::Vec3;
use crate::utilities::ray::{Intersectable, Ray, Intersection};

use super::triangle::TriangleColor;

pub enum BVH{
    Branch{left: Box<BVH>, right: Box<BVH>, aabb: AABB},
    Leaf(Triangle)
}

fn bounding(triangles: &Vec<Triangle>) -> AABB{
    if triangles.len() <= 0 {
        return AABB::new(Vec3::default(), Vec3::default());
    }
    let mut bounds = triangles[0].bounds();
    for triangle in triangles{
        bounds = bounds.union(triangle.bounds());
    }
    bounds
}

impl BVH{
    pub fn from_triangles(triangles: &Vec<Triangle>) -> Option<Self>{
        if triangles.len() == 0 {
            return None;
        }
        if triangles.len() == 1 {
            let triangle = triangles.iter().next().unwrap().clone();
            return Some(BVH::Leaf(triangle));
        }
        let bounds = bounding(&triangles);
        let maxis = bounds.max_axis();
        let mut along_maxis: Vec<_> = triangles.iter().map(|t|(t.centroid().get(maxis), t.clone())).collect();
        let index = along_maxis.len()/2;
        along_maxis.select_nth_unstable_by(index, |a, b|a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
        let along_maxis: Vec<_> = along_maxis.into_iter().map(|(_, t)|t).collect();
        let (a, b) = along_maxis.split_at(index);
        let a = BVH::from_triangles(&a.to_vec())?;
        let b = BVH::from_triangles(&b.to_vec())?;
        Some(BVH::Branch {
            left: Box::new(a),
            right: Box::new(b),
            aabb: bounds
        })
    }
}

impl Intersectable for BVH {
    type C = TriangleColor;
    fn intersect(&self, ray: Ray) -> Option<Intersection<Self::C>> {
        match self{
            BVH::Leaf(triangle) => triangle.intersect(ray),
            BVH::Branch { left, right, aabb } => {
                if let None = aabb.intersect(ray) {
                    return None;
                }
                let a = left.intersect(ray);
                let b = right.intersect(ray);
                match(a, b){
                    (None, None) => None,
                    (None, Some(b)) => Some(b),
                    (Some(a), None) => Some(a),
                    (Some(a), Some(b)) if a.get_distance() < b.get_distance() => Some(a),
                    (Some(_), Some(b)) => Some(b),
                }
            },
        }
    }
}