use asset_id::{PropID, TextureID};
use bincode::ErrorKind;
use cgmath::{Vector2, Vector3};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub model: Model,
    pub props: Vec<Prop>,
}

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector2<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub solids: Vec<Solid>,
}

#[derive(Serialize, Deserialize)]
pub struct Solid {
    pub faces: [Face; 6],
    pub points: [Point; 8],
}

#[derive(Serialize, Deserialize)]
pub struct Face {
    pub texture_id: TextureID,
    pub points: [PointID; 4],
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct PointID(pub usize);

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub position: Vector3<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Prop {
    pub prop_id: PropID,
    pub position: Vector3<i32>,
    pub rotation: Vector3<i32>,
}

impl Scene {
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        Ok(bincode::serialize(self)?)
    }

    pub fn decode(&self, buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(bincode::deserialize(buf)?)
    }
}

impl From<PointID> for usize {
    fn from(value: PointID) -> Self {
        value.0 as Self
    }
}

#[derive(Error, Debug)]
#[error("{0}")]
pub struct EncodeError(#[from] Box<ErrorKind>);

#[derive(Error, Debug)]
#[error("{0}")]
pub struct DecodeError(#[from] Box<ErrorKind>);
