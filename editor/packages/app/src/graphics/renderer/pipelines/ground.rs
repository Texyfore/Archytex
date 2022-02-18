use std::mem::size_of;

use gpu::{
    vertex_attr_array, Gpu, Pipeline, PipelineConfig, PipelineInput, PipelineTopology, Surface,
    VertexBufferLayout, VertexStepMode,
};

use crate::graphics::structures::GroundVertex;

pub fn pipeline(gpu: &Gpu, surface: &Surface) -> Pipeline {
    gpu.create_pipeline(
        surface,
        &PipelineConfig {
            shader_source: include_str!("shaders/ground.wgsl"),
            inputs: &[
                PipelineInput::Uniform, // Camera
                PipelineInput::Texture, // Texture
            ],
            vertex_buffers: &[VertexBufferLayout {
                array_stride: size_of::<GroundVertex>() as u64,
                step_mode: VertexStepMode::Vertex,
                attributes: &vertex_attr_array![
                    0 => Float32x3, // Position
                    1 => Float32x2, // Texcoord
                ],
            }],
            topology: PipelineTopology::Triangles,
        },
    )
}
