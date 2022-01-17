use gpu::{data::Buffer, BufferUsages};

use crate::Renderer;

pub struct Mesh {
    pub(crate) vertices: Buffer<mesh::Vertex>,
    pub(crate) triangles: Buffer<[u16; 3]>,
}

impl Renderer {
    pub fn create_mesh(&self, vertices: &[mesh::Vertex], triangles: &[[u16; 3]]) -> Mesh {
        Mesh {
            vertices: self.gpu.create_buffer(vertices, BufferUsages::VERTEX),
            triangles: self.gpu.create_buffer(triangles, BufferUsages::INDEX),
        }
    }
}
