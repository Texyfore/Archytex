use std::fmt::Debug;

use cgmath::{Vector2, Vector3};

use crate::TextureID;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PropID(pub u32);

impl Debug for PropID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

pub struct Prop {
    pub bounds: BoundingBox,
    pub meshes: Vec<Mesh>,
}

pub struct BoundingBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

pub struct Mesh {
    pub texture: TextureID,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<[u16; 3]>,
}

pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
}
