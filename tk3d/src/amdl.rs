use serde::{Deserialize, Serialize};

use crate::{error::DecodeError, TexturedMesh};

#[derive(Serialize, Deserialize)]
pub struct PropModel {
    pub mesh: TexturedMesh,
}

impl PropModel {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        bincode::deserialize(buf).map_err(|source| DecodeError { source })
    }
}
