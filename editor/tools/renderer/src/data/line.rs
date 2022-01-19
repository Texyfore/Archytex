use bytemuck::{Pod, Zeroable};
use cgmath::Vector3;
use gpu::{data::Buffer, BufferUsages};

use crate::Renderer;

pub struct Mesh {
    pub(crate) vertices: Buffer<Vertex>,
}

impl Renderer {
    pub fn create_lines(&self, vertices: &[Vertex]) -> Mesh {
        Mesh {
            vertices: self.gpu.create_buffer(vertices, BufferUsages::VERTEX),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: [f32; 3],
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}
