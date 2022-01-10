use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub model: Model,
    pub props: Vec<(u32, Prop)>,
}

#[derive(Serialize, Deserialize)]
pub struct Camera {
    pub position: Vector3,
    pub rotation: Vector3,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub points: Vec<(u32, Point)>,
    pub faces: Vec<(u32, Face)>,
    pub solids: Vec<(u32, Solid)>,
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub position: Vector3,
}

#[derive(Serialize, Deserialize)]
pub struct Face {
    pub points: [u32; 4],
    pub texture_id: TextureID,
}

#[derive(Serialize, Deserialize)]
pub struct Solid {
    pub faces: [u32; 6],
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

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    pub texture: TextureID,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub bounds: BoundingBox,
}

#[derive(Serialize, Deserialize)]
pub struct Vertex {
    pub position: Vector3,
    pub normal: Vector3,
    pub texcoord: Vector2,
}

#[derive(Serialize, Deserialize)]
pub struct Triangle {
    pub indices: [u16; 3],
}

#[derive(Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

impl Mesh {
    pub fn encode(self) -> Option<Vec<u8>> {
        bincode::serialize(&self).ok()
    }

    pub fn decode(buf: &[u8]) -> Option<Self> {
        bincode::deserialize(buf).ok()
    }
}
