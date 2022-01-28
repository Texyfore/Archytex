use wgpu::{
    vertex_attr_array, CompareFunction, DepthStencilState, Face, FragmentState, FrontFace,
    MultisampleState, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, TextureFormat,
    VertexBufferLayout, VertexState, VertexStepMode,
};

use crate::{data::UniformLayout, handle::GpuHandle};

pub struct GridPipeline {
    pub(crate) inner: RenderPipeline,
}

impl GpuHandle {
    pub fn create_grid_pipeline(&self, uniform_layout: &UniformLayout) -> GridPipeline {
        let module = self.device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(include_str!("grid.wgsl").into()),
        });

        GridPipeline {
            inner: self
                .device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: None,
                    layout: Some(
                        &self
                            .device
                            .create_pipeline_layout(&PipelineLayoutDescriptor {
                                label: None,
                                bind_group_layouts: &[&uniform_layout.inner],
                                push_constant_ranges: &[],
                            }),
                    ),
                    vertex: VertexState {
                        module: &module,
                        entry_point: "vs_main",
                        buffers: &[
                            VertexBufferLayout {
                                array_stride: 12,
                                step_mode: VertexStepMode::Vertex,
                                attributes: &vertex_attr_array![
                                    0 => Float32x3,
                                ],
                            },
                        ],
                    },
                    primitive: PrimitiveState {
                        topology: PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: FrontFace::Ccw,
                        cull_mode: Some(Face::Back),
                        unclipped_depth: false,
                        polygon_mode: PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: Some(DepthStencilState {
                        format: TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: CompareFunction::Less,
                        stencil: Default::default(),
                        bias: Default::default(),
                    }),
                    multisample: MultisampleState::default(),
                    fragment: Some(FragmentState {
                        module: &module,
                        entry_point: "fs_main",
                        targets: &[self.surface_format.into()],
                    }),
                    multiview: None,
                }),
        }
    }
}
