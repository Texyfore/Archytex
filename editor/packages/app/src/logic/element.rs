use asset::TextureID;
use cgmath::Vector3;

use crate::graphics::SolidMesh;

pub enum ElementKind {
    Solid,
    Face,
    Point,
    Prop,
}

pub struct Solid {
    faces: [Face; 6],
    points: [Point; 8],
    selected: bool,
    mesh: SolidMesh,
}

pub struct Face {
    texture: TextureID,
    indices: [usize; 4],
    selected: bool,
}

pub struct Point {
    position: Vector3<i32>,
    selected: bool,
}
