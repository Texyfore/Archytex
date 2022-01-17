use asset_id::TextureID;
use bincode::ErrorKind;
use cgmath::Vector3;
use mesh::Mesh;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub texture_id: TextureID,
    pub bounding_box: BoundingBox,
    pub mesh: Mesh,
}

impl Model {
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok(bincode::serialize(self)?)
    }

    pub fn decode(&self, buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::deserialize(buf)?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct EncodeError(#[from] Box<ErrorKind>);

#[derive(Error, Debug)]
#[error("{0}")]
pub struct DecodeError(#[from] Box<ErrorKind>);
