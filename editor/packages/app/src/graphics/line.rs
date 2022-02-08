use std::mem::size_of;

use bytemuck::{Pod, Zeroable};
use cgmath::Vector3;
use gpu::{
    vertex_attr_array, Buffer, Gpu, Pipeline, PipelineConfig, PipelineInput, PipelineTopology, Res,
    Surface, VertexBufferLayout, VertexStepMode,
};

use super::Share;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: [f32; 3],
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

pub struct Object {
    pub(super) vertices: Res<Buffer<Vertex>>,
}

impl Share for Object {
    fn share(&self) -> Self {
        Self {
            vertices: self.vertices.share(),
        }
    }
}

pub(super) fn pipeline(gpu: &Gpu, surface: &Surface) -> Pipeline {
    gpu.create_pipeline(
        surface,
        &PipelineConfig {
            shader_source: include_str!("shaders/line.wgsl"),
            inputs: &[
                PipelineInput::Uniform, // Camera
            ],
            vertex_buffers: &[VertexBufferLayout {
                array_stride: size_of::<Vertex>() as u64,
                step_mode: VertexStepMode::Vertex,
                attributes: &vertex_attr_array![
                    0 => Float32x3, // Position
                    1 => Float32x3, // Color
                ],
            }],
            topology: PipelineTopology::Lines,
        },
    )
}
