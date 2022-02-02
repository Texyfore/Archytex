use bytemuck::{Pod, Zeroable};
use cgmath::{Vector2, Vector3};
use gpu::{data::Buffer, BufferUsages};

use crate::Renderer;

pub struct Mesh {
    pub(crate) vertices: Buffer<Vertex>,
    pub(crate) triangles: Buffer<[u16; 3]>,
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

impl Renderer {
    pub fn create_prop(&self, vertices: &[Vertex], triangles: &[[u16; 3]]) -> Mesh {
        Mesh {
            vertices: self.gpu.create_buffer(vertices, BufferUsages::VERTEX),
            triangles: self.gpu.create_buffer(triangles, BufferUsages::INDEX),
        }
    }
}
