use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Vector3};
use gpu::{data::Buffer, BufferUsages};

use crate::Renderer;

pub struct Instances {
    pub(crate) buffer: Buffer<Instance>,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Instance {
    matrix: [[f32; 4]; 4],
    color: [f32; 4],
}

impl Instance {
    pub fn new(matrix: Matrix4<f32>, color: [f32; 3]) -> Self {
        Self {
            matrix: matrix.into(),
            color: [color[0], color[1], color[2], 1.0],
        }
    }
}

pub struct Mesh {
    pub(crate) vertices: Buffer<Vertex>,
    pub(crate) triangles: Buffer<[u16; 3]>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vector3<f32>,
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

impl Renderer {
    pub fn create_gizmo_instances(&self, instances: &[Instance]) -> Instances {
        Instances {
            buffer: self.gpu.create_buffer(instances, BufferUsages::VERTEX),
        }
    }

    pub fn create_gizmo_mesh(&self, vertices: &[Vertex], triangles: &[[u16; 3]]) -> Mesh {
        Mesh {
            vertices: self.gpu.create_buffer(vertices, BufferUsages::VERTEX),
            triangles: self.gpu.create_buffer(triangles, BufferUsages::INDEX),
        }
    }
}
