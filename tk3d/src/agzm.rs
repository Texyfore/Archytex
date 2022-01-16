use cgmath::Vector3;
use serde::{Deserialize, Serialize};

use crate::{error::DecodeError, Triangle};

#[derive(Serialize, Deserialize)]
pub struct Gizmo {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl Gizmo {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::deserialize(buf)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Vertex {
    pub position: Vector3<f32>,
}
