use gpu::{data::Buffer, BufferUsages};
use tk3d::{Triangle, Vertex};

use crate::Renderer;

pub struct Mesh {
    pub(crate) vertices: Buffer<Vertex>,
    pub(crate) triangles: Buffer<Triangle>,
}

impl Renderer {
    pub fn create_mesh(&self, vertices: &[Vertex], triangles: &[Triangle]) -> Mesh {
        Mesh {
            vertices: self.gpu.create_buffer(vertices, BufferUsages::VERTEX),
            triangles: self.gpu.create_buffer(triangles, BufferUsages::INDEX),
        }
    }
}
