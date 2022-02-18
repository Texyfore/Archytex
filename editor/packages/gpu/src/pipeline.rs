use wgpu::{
    CompareFunction, DepthStencilState, Face, FragmentState, FrontFace, PipelineLayoutDescriptor,
    PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline, RenderPipelineDescriptor,
    ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexState,
};

use crate::{Gpu, Surface};

pub use wgpu::{vertex_attr_array, VertexBufferLayout, VertexStepMode};

pub struct Pipeline {
    pub(crate) pipeline: RenderPipeline,
}

pub struct PipelineConfig<'a, 'b, 'c> {
    pub shader_source: &'a str,
    pub inputs: &'b [PipelineInput],
    pub vertex_buffers: &'c [VertexBufferLayout<'c>],
    pub topology: PipelineTopology,
}

pub enum PipelineInput {
    Texture,
    Uniform,
}

pub enum PipelineTopology {
    Triangles,
    Lines,
}

impl Gpu {
    pub fn create_pipeline(&self, surface: &Surface, config: &PipelineConfig) -> Pipeline {
        let shader = self.device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(config.shader_source.into()),
        });

        let bind_group_layouts = config
            .inputs
            .iter()
            .map(|input| match input {
                PipelineInput::Texture => &self.texture_layout,
                PipelineInput::Uniform => &self.uniform_layout,
            })
            .collect::<Vec<_>>();

        Pipeline {
            pipeline: self
                .device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: None,
                    layout: Some(
                        &self
                            .device
                            .create_pipeline_layout(&PipelineLayoutDescriptor {
                                label: None,
                                bind_group_layouts: &bind_group_layouts,
                                push_constant_ranges: &[],
                            }),
                    ),
                    vertex: VertexState {
                        module: &shader,
                        entry_point: "vertex",
                        buffers: config.vertex_buffers,
                    },
                    primitive: PrimitiveState {
                        topology: match config.topology {
                            PipelineTopology::Triangles => PrimitiveTopology::TriangleList,
                            PipelineTopology::Lines => PrimitiveTopology::LineList,
                        },
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
                    multisample: Default::default(),
                    fragment: Some(FragmentState {
                        module: &shader,
                        entry_point: "fragment",
                        targets: &[surface.format.into()],
                    }),
                    multiview: None,
                }),
        }
    }
}
