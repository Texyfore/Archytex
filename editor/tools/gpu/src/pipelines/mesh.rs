use wgpu::{
    vertex_attr_array, Face, FragmentState, FrontFace, IndexFormat, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, VertexBufferLayout,
    VertexState, VertexStepMode,
};

use crate::handle::GpuHandle;

pub struct MeshPipeline {
    pub(crate) inner: RenderPipeline,
}

impl GpuHandle {
    pub fn create_mesh_pipeline(&self) -> MeshPipeline {
        let module = self.device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(include_str!("mesh.wgsl").into()),
        });

        MeshPipeline {
            inner: self
                .device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: None,
                    layout: Some(
                        &self
                            .device
                            .create_pipeline_layout(&PipelineLayoutDescriptor {
                                label: None,
                                bind_group_layouts: &[],
                                push_constant_ranges: &[],
                            }),
                    ),
                    vertex: VertexState {
                        module: &module,
                        entry_point: "main",
                        buffers: &[VertexBufferLayout {
                            array_stride: 24,
                            step_mode: VertexStepMode::Vertex,
                            attributes: &vertex_attr_array![
                                0 => Float32x3,
                                1 => Float32x3,
                                2 => Float32x2,
                            ],
                        }],
                    },
                    primitive: PrimitiveState {
                        topology: PrimitiveTopology::TriangleList,
                        strip_index_format: Some(IndexFormat::Uint16),
                        front_face: FrontFace::Ccw,
                        cull_mode: Some(Face::Back),
                        unclipped_depth: false,
                        polygon_mode: PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    fragment: Some(FragmentState {
                        module: &module,
                        entry_point: "main",
                        targets: &[self.surface_format.into()],
                    }),
                    multiview: None,
                }),
        }
    }
}
