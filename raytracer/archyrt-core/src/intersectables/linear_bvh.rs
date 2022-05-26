use crate::{utilities::{math::Vector, ray::{Intersectable, Ray, Intersection}}, intersectables::triangle};

use super::{aabb::AABB, triangle::{Triangle, TriangleColor}, bvh::BVH};

pub enum LinearBVHNode{
    Branch{
        aabb: AABB,
    },
    Leaf{
        triangle: Triangle
    }
}

pub struct LinearBVH(pub Vec<Option<LinearBVHNode>>);

impl From<BVH> for LinearBVH{
    fn from(bvh: BVH) -> Self {
        let l = (1usize<<bvh.depth())-1;
        let mut buf: Vec<Option<LinearBVHNode>> = (0..l).map(|_|None).collect();
        fn generate(buf: &mut Vec<Option<LinearBVHNode>>, bvh: &BVH, index: usize){
            match bvh {
                BVH::Branch { left, right, aabb } => {
                    buf[index] = Some(LinearBVHNode::Branch { aabb: *aabb });
                    let l = index*2+1;
                    let r = l+1;
                    generate(buf, left, l);
                    generate(buf, right, r);
                },
                BVH::Leaf(triangle) => {
                    buf[index] = Some(LinearBVHNode::Leaf { triangle: triangle.clone() });
                },
            }
        }
        generate(&mut buf, &bvh, 0);
        LinearBVH(buf)
    }
}

impl Intersectable for LinearBVH{
    type C = TriangleColor;

    fn intersect(&self, ray: Ray) -> Option<Intersection<Self::C>> {
        fn intersect(buf: &LinearBVH, index: usize, ray: Ray)->Option<Intersection<TriangleColor>>{
            let s = buf.0[index].as_ref()?;
            match s{
                LinearBVHNode::Branch { aabb } => {
                    aabb.intersect(ray)?;
                    let l = index*2+1;
                    let r = l+1;
                    let l = intersect(buf, l, ray);
                    let r = intersect(buf, r, ray);
                    match (l, r) {
                        (None, None) => None,
                        (Some(a), None) => Some(a),
                        (None, Some(b)) => Some(b),
                        (Some(a), Some(b)) if a.get_distance() < b.get_distance() => Some(a),
                        (Some(a), Some(b)) => Some(b)
                    }
                },
                LinearBVHNode::Leaf { triangle } => {
                    triangle.intersect(ray)
                },
            }
        }
        intersect(self, 0, ray)
    }
}