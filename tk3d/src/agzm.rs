use cgmath::Vector3;
use serde::{Deserialize, Serialize};

use crate::{error::DecodeError, Triangle};

#[derive(Serialize, Deserialize)]
pub struct GizmoMesh {
    pub vertices: Vec<GizmoVertex>,
    pub triangles: Vec<Triangle>,
}

impl GizmoMesh {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::deserialize(buf)?)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GizmoVertex {
    pub position: Vector3<f32>,
}
