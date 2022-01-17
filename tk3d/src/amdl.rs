use serde::{Deserialize, Serialize};
use cgmath::Vector3;

use crate::{error::DecodeError, Mesh};

#[derive(Serialize, Deserialize)]
pub struct PropModel {
    pub bounding_box: BoundingBox,
    pub mesh: Mesh,
}

#[derive(Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

impl PropModel {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::deserialize(buf)?)
    }
}
