use gpu::{data::Buffer, BufferUsages};

use crate::Renderer;

pub struct Mesh {
    pub(crate) vertices: Buffer<mesh::Vertex>,
    pub(crate) triangles: Buffer<[u16; 3]>,
}

impl Renderer {
    pub fn create_mesh(&self, mesh: &mesh::Mesh) -> Mesh {
        Mesh {
            vertices: self.gpu.create_buffer(&mesh.vertices, BufferUsages::VERTEX),
            triangles: self.gpu.create_buffer(&mesh.triangles, BufferUsages::INDEX),
        }
    }
}
