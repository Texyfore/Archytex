use asset_id::TextureID;
use bincode::ErrorKind;
use mesh::Mesh;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub texture_id: TextureID,
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

#[derive(Error, Debug)]
#[error("couldn't encode Model: {0}")]
pub struct EncodeError(#[from] Box<ErrorKind>);

#[derive(Error, Debug)]
#[error("couldn't decode Model: {0}")]
pub struct DecodeError(#[from] Box<ErrorKind>);
