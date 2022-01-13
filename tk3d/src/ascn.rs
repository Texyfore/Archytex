use cgmath::Vector3;
use serde::{Deserialize, Serialize};

use crate::{error::DecodeError, PropID, TextureID};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub model: Model,
    pub props: Vec<(u32, Prop)>,
}

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub points: Vec<(u32, Point)>,
    pub faces: Vec<(u32, Face)>,
    pub solids: Vec<(u32, Solid)>,
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub position: Vector3<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Face {
    pub points: [u32; 4],
    pub texture: TextureID,
}

#[derive(Serialize, Deserialize)]
pub struct Solid {
    pub faces: [u32; 6],
}

#[derive(Serialize, Deserialize)]
pub struct Prop {
    pub id: PropID,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
}

impl Scene {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        bincode::deserialize(buf).map_err(|source| DecodeError { source })
    }
}
