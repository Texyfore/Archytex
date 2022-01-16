use serde::{Deserialize, Serialize};

use crate::{error::DecodeError, Mesh};

#[derive(Serialize, Deserialize)]
pub struct PropModel {
    pub mesh: Mesh,
}

impl PropModel {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::deserialize(buf)?)
    }
}
