use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub model: Model,
    pub props: Vec<Prop>,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub faces: Vec<Face>,
}

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub position: Vector3,
    pub rotation: Vector3,
}

#[derive(Serialize, Deserialize)]
pub struct Face {
    pub vertices: [Vertex; 4],
    pub triangles: [Triangle; 2],
    pub texture_id: TextureID,
}

#[derive(Serialize, Deserialize)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub texcord: Vector2,
}

#[derive(Serialize, Deserialize)]
pub struct Triangle {
    pub a: u16,
    pub b: u16,
    pub c: u16,
}

#[derive(Serialize, Deserialize)]
pub struct TextureID(u32);

#[derive(Serialize, Deserialize)]
pub struct Prop {
    pub id: PropID,
    pub position: Vector3,
    pub rotation: Vector3,
}

#[derive(Serialize, Deserialize)]
pub struct PropID(u32);

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

impl From<PropID> for u32 {
    fn from(val: PropID) -> Self {
        val.0
    }
}

impl From<TextureID> for u32 {
    fn from(val: TextureID) -> Self {
        val.0
    }
}

impl Scene {
    pub fn encode(self) -> Option<Vec<u8>> {
        bincode::serialize(&self).ok()
    }

    pub fn decode(buf: &[u8]) -> Option<Self> {
        bincode::deserialize(buf).ok()
    }
}
