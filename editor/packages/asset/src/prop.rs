use std::fmt::Debug;

use bytemuck::{Pod, Zeroable};
use cgmath::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

use crate::TextureID;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PropID(pub u32);

impl Debug for PropID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Prop {
    pub bounds: BoundingBox,
    pub meshes: Vec<PropMesh>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct BoundingBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct PropMesh {
    pub texture: TextureID,
    pub vertices: Vec<PropVertex>,
    pub triangles: Vec<[u16; 3]>,
}

#[repr(C)]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PropVertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
}

unsafe impl Zeroable for PropVertex {}
unsafe impl Pod for PropVertex {}
