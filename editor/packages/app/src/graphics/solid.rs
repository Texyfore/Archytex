use std::mem::size_of;

use asset::TextureID;
use bytemuck::{Pod, Zeroable};
use cgmath::{Vector2, Vector3};
use gpu::{
    vertex_attr_array, Buffer, Gpu, Pipeline, PipelineConfig, PipelineInput, PipelineTopology, Res,
    Surface, VertexBufferLayout, VertexStepMode,
};

use super::Share;

pub struct Mesh<'v, 't> {
    pub texture: TextureID,
    pub vertices: &'v [Vertex],
    pub triangles: &'t [[u16; 3]],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
    pub tint: [f32; 4],
}

unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

pub struct Object {
    pub(super) texture: TextureID,
    pub(super) vertices: Res<Buffer<Vertex>>,
    pub(super) triangles: Res<Buffer<[u16; 3]>>,
}

impl Share for Object {
    fn share(&self) -> Self {
        Self {
            texture: self.texture,
            vertices: self.vertices.share(),
            triangles: self.triangles.share(),
        }
    }
}

pub(super) fn pipeline(gpu: &Gpu, surface: &Surface) -> Pipeline {
    gpu.create_pipeline(
        surface,
        &PipelineConfig {
            shader_source: include_str!("shaders/solid.wgsl"),
            inputs: &[
                PipelineInput::Uniform, // Camera
                PipelineInput::Texture, // Texture
            ],
            vertex_buffers: &[VertexBufferLayout {
                array_stride: size_of::<Vertex>() as u64,
                step_mode: VertexStepMode::Vertex,
                attributes: &vertex_attr_array![
                    0 => Float32x3, // Position
                    1 => Float32x3, // Normal
                    2 => Float32x2, // Texcoord
                    3 => Float32x4, // Tint
                ],
            }],
            topology: PipelineTopology::Triangles,
        },
    )
}
