use crate::utilities::math::{Vector, Vec3};

pub fn tonemap_fragment(c: Vec3)->Vec3{
    let c = c*4.0;
    let c = c/(c+Vector::from_single(1.0));
    let c = c.powf(1.0/2.2);
    c
}