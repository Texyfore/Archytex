use bincode::ErrorKind;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use gizmo::{Mesh, Vertex};

#[derive(Serialize, Deserialize)]
pub struct Gizmo {
    pub mesh: Mesh,
}

impl Gizmo {
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok(bincode::serialize(self)?)
    }

    pub fn decode(&self, buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::deserialize(buf)?)
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct EncodeError(#[from] Box<ErrorKind>);

#[derive(Error, Debug)]
#[error("{0}")]
pub struct DecodeError(#[from] Box<ErrorKind>);
