use std::rc::Rc;

use asset::PropID;
use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix};
use gpu::{
    vertex_attr_array, Gpu, Pipeline, PipelineConfig, PipelineInput, PipelineTopology, Surface,
    Uniform, VertexBufferLayout, VertexStepMode,
};

use super::Share;

pub struct Object {
    pub(super) prop: PropID,
    pub(super) uniform: Rc<Uniform<Properties>>,
}

impl Share for Object {
    fn share(&self) -> Self {
        Self {
            prop: self.prop,
            uniform: self.uniform.clone(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Properties {
    pub transform: Matrix4<f32>,
    pub tint: [f32; 4],
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            transform: Matrix4::identity(),
            tint: [0.0; 4],
        }
    }
}

unsafe impl Zeroable for Properties {}
unsafe impl Pod for Properties {}

pub(super) fn pipeline(gpu: &Gpu, surface: &Surface) -> Pipeline {
    gpu.create_pipeline(
        surface,
        &PipelineConfig {
            shader_source: include_str!("shaders/prop.wgsl"),
            inputs: &[
                PipelineInput::Uniform, // Camera
                PipelineInput::Uniform, // Properties
                PipelineInput::Texture, // Texture
            ],
            vertex_buffers: &[VertexBufferLayout {
                array_stride: 32,
                step_mode: VertexStepMode::Vertex,
                attributes: &vertex_attr_array![
                    0 => Float32x3, // Position
                    1 => Float32x3, // Normal
                    2 => Float32x2, // Texcoord
                ],
            }],
            topology: PipelineTopology::Triangles,
        },
    )
}
