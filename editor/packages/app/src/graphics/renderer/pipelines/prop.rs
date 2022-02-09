use std::mem::size_of;

use asset::PropVertex;
use gpu::{
    vertex_attr_array, Gpu, Pipeline, PipelineConfig, PipelineInput, PipelineTopology, Surface,
    VertexBufferLayout, VertexStepMode,
};

pub fn pipeline(gpu: &Gpu, surface: &Surface) -> Pipeline {
    gpu.create_pipeline(
        surface,
        &PipelineConfig {
            shader_source: include_str!("shaders/prop.wgsl"),
            inputs: &[
                PipelineInput::Uniform, // Camera
                PipelineInput::Uniform, // Data
                PipelineInput::Texture, // Texture
            ],
            vertex_buffers: &[VertexBufferLayout {
                array_stride: size_of::<PropVertex>() as u64,
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
