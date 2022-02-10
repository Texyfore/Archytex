use std::mem::size_of;

use asset::GizmoVertex;
use gpu::{
    vertex_attr_array, Gpu, Pipeline, PipelineConfig, PipelineInput, PipelineTopology, Surface,
    VertexBufferLayout, VertexStepMode,
};

use crate::graphics::structures::GizmoInstance;

pub fn pipeline(gpu: &Gpu, surface: &Surface) -> Pipeline {
    gpu.create_pipeline(
        surface,
        &PipelineConfig {
            shader_source: include_str!("shaders/gizmo.wgsl"),
            inputs: &[
                PipelineInput::Uniform, // Camera
            ],
            vertex_buffers: &[
                VertexBufferLayout {
                    array_stride: size_of::<GizmoVertex>() as u64,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &vertex_attr_array![
                        0 => Float32x3, // Position
                    ],
                },
                VertexBufferLayout {
                    array_stride: size_of::<GizmoInstance>() as u64,
                    step_mode: VertexStepMode::Instance,
                    attributes: &vertex_attr_array![
                        1 => Float32x4, // Transform (mat4x4)
                        2 => Float32x4,
                        3 => Float32x4,
                        4 => Float32x4,
                        5 => Float32x3, // Color
                    ],
                },
            ],
            topology: PipelineTopology::Triangles,
        },
    )
}
