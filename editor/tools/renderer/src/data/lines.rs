use bytemuck::{Pod, Zeroable};
use gpu::{data::buffer::Buffer, BufferUsages};
use tk3d::math::Vector3;

use crate::Renderer;

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
