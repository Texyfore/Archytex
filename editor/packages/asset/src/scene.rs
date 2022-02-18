use cgmath::{Quaternion, Vector2, Vector3};
use serde::{Deserialize, Serialize};

use crate::{PropID, TextureID};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub world: World,
}

#[derive(Serialize, Deserialize)]
pub struct World {
    pub solids: Vec<Solid>,
    pub props: Vec<Prop>,
}

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub position: Vector3<f32>,
    pub rotation: Vector2<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Solid {
    pub points: [Point; 8],
    pub faces: [Face; 6],
}

#[derive(Serialize, Deserialize)]
pub struct Face {
    pub texture: TextureID,
    pub indices: [u32; 4],
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub position: Vector3<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Prop {
    pub asset: PropID,
    pub position: Vector3<i32>,
    pub rotation: Quaternion<f32>,
}
