use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub model: Model,
    pub props: Vec<Prop>,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub meshes: Vec<Mesh>,
}

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub position: Vector3,
    pub rotation: Vector3,
}

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub texture_id: TextureID,
}

#[derive(Serialize, Deserialize)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub texcoord: Vector2,
}

#[derive(Serialize, Deserialize)]
pub struct Triangle {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

#[derive(Serialize, Deserialize)]
pub struct TextureID(pub u32);

#[derive(Serialize, Deserialize)]
pub struct Prop {
    pub id: PropID,
    pub position: Vector3,
    pub rotation: Vector3,
}

#[derive(Serialize, Deserialize)]
pub struct PropID(pub u32);

#[derive(Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Scene {
    pub fn encode(self) -> Option<Vec<u8>> {
        bincode::serialize(&self).ok()
    }

    pub fn decode(buf: &[u8]) -> Option<Self> {
        bincode::deserialize(buf).ok()
    }
}

impl Model {
    pub fn encode(self) -> Option<Vec<u8>> {
        bincode::serialize(&self).ok()
    }

    pub fn decode(buf: &[u8]) -> Option<Self> {
        bincode::deserialize(buf).ok()
    }
}
