use crate::{utilities::{math::Vector, ray::{Intersectable, Ray, Intersection}}, intersectables::triangle};

use super::{aabb::AABB, triangle::{Triangle, TriangleColor}, bvh::BVH};

pub enum LinearBVHNode{
    Branch{
        aabb: AABB,
        right: usize
    },
    Leaf{
        triangle: Triangle
    }
}

pub struct LinearBVH(pub Vec<LinearBVHNode>);

impl From<BVH> for LinearBVH{
    fn from(bvh: BVH) -> Self {
        let mut buf: Vec<LinearBVHNode> = Vec::new();
        fn generate(buf: &mut Vec<LinearBVHNode>, bvh: &BVH){
            match bvh {
                BVH::Branch { left, right, aabb } => {
                    buf.push(LinearBVHNode::Branch { aabb: *aabb, right: 0 });
                    let idx = buf.len()-1;
                    generate(buf, left);
                    let right_idx = buf.len();
                    if let LinearBVHNode::Branch { right, .. } = buf.get_mut(idx).unwrap() {
                        *right = right_idx;
                    }
                    generate(buf, right);
                },
                BVH::Leaf(triangle) => {
                    buf.push(LinearBVHNode::Leaf { triangle: triangle.clone() })
                },
            }
        }
        generate(&mut buf, &bvh);
        LinearBVH(buf)
    }
}

impl Intersectable for LinearBVH{
    type C = TriangleColor;

    fn intersect(&self, ray: Ray) -> Option<Intersection<Self::C>> {
        fn intersect(buf: &LinearBVH, index: usize, ray: Ray)->Option<Intersection<TriangleColor>>{
            let s = &buf.0[index];
            match s{
                LinearBVHNode::Branch { aabb, right } => {
                    aabb.intersect(ray)?;
                    let l = intersect(buf, index+1, ray);
                    let r = intersect(buf, *right, ray);
                    match (l, r) {
                        (None, None) => None,
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(b),
                        (Some(a), Some(b)) if a.get_distance() < b.get_distance() => Some(a),
                        (Some(a), Some(b)) => Some(b)
                    }
                },
                LinearBVHNode::Leaf { triangle } => triangle.intersect(ray),
            }
        }
        intersect(self, 0, ray)
    }
}