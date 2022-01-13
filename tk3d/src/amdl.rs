use cgmath::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

use crate::{error::DecodeError, id::TextureID};

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    pub texture: TextureID,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

#[derive(Serialize, Deserialize)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Triangle {
    pub indices: [u16; 3],
}

impl Mesh {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        bincode::deserialize(buf).map_err(|source| DecodeError { source })
    }
}