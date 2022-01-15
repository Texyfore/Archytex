use bytemuck::{Pod, Zeroable};
use gpu::{
    data::{buffer::Buffer, uniform::Uniform},
    BufferUsages,
};
use tk3d::{
    math::{Matrix4, SquareMatrix, Vector3},
    Triangle, Vertex,
};

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

pub struct Lines {
    pub(crate) vertices: Buffer<LineVertex>,
}

impl Renderer {
    pub fn create_lines(&self, vertices: &[LineVertex]) -> Lines {
        Lines {
            vertices: self.gpu.create_buffer(vertices, BufferUsages::VERTEX),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LineVertex {
    pub position: Vector3<f32>,
    pub color: [f32; 3],
}

unsafe impl Zeroable for LineVertex {}
unsafe impl Pod for LineVertex {}

pub struct Transform {
    pub(crate) uniform: Uniform<[[f32; 4]; 4]>,
}

impl Renderer {
    pub fn create_transform(&self) -> Transform {
        let uniform = self.gpu.create_uniform(&self.uniform_layout);
        self.gpu.set_uniform(&uniform, &Matrix4::identity().into());
        Transform { uniform }
    }

    pub fn set_transform(&self, transform: &Transform, matrix: Matrix4<f32>) {
        self.gpu.set_uniform(&transform.uniform, &matrix.into());
    }
}
