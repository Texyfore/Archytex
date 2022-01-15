pub mod amdl;
pub mod ascn;
pub mod error;
pub mod proc;

pub use cgmath as math;

use math::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    pub texture: TextureID,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct TextureID(pub u32);

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PropID(pub u32);

#[derive(Serialize, Deserialize, Clone)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Triangle {
    pub indices: [u16; 3],
}

#[cfg(feature="bytemuck")]
unsafe impl bytemuck::Pod for Vertex {}

#[cfg(feature="bytemuck")]
unsafe impl bytemuck::Pod for Triangle {}