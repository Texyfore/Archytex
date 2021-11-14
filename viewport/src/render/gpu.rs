use std::{iter::once, marker::PhantomData, mem::size_of, num::NonZeroU32};

use bytemuck::{cast_slice, Pod};
use futures_lite::future;
use image::{DynamicImage, EncodableLayout, GenericImageView};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    *,
};
use winit::window::Window;

use crate::render::data::BrushVertex;

use super::{
    data::{LineVertex, Triangle},
    CameraBlock, TransformBlock, MSAA_SAMPLE_COUNT,
};

pub(super) struct Context {
    surface: Surface,
    surface_format: TextureFormat,
    device: Device,
    queue: Queue,
}

pub(super) struct TypedBuffer<T: Pod> {
    inner: Buffer,
    len: u64,
    _p: PhantomData<T>,
}

pub(super) struct LinePipeline {
    inner: RenderPipeline,
}

pub(super) struct BrushPipeline {
    inner: RenderPipeline,
}

pub(super) struct Frame {
    texture: SurfaceTexture,
    view: TextureView,
    encoder: CommandEncoder,
}

pub(super) struct Pass<'a> {
    inner: RenderPass<'a>,
}

pub(super) struct UniformBufferLayout {
    inner: BindGroupLayout,
}

pub(super) struct UniformBufferGroup<T: Pod> {
    inner: BindGroup,
    buffer: TypedBuffer<T>,
}

pub(super) struct TextureLayout {
    inner: BindGroupLayout,
}

pub(super) struct TextureGroup {
    inner: BindGroup,
}

pub(super) struct MsaaFramebuffer {
    view: TextureView,
}

impl Context {
    pub fn new(window: &Window) -> Self {
        let instance = Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let (surface_format, device, queue) = future::block_on(async {
            let adapter = instance
                .request_adapter(&RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await
                .unwrap();

            let surface_format = surface
                .get_preferred_format(&adapter)
                .unwrap_or(TextureFormat::Bgra8UnormSrgb);

            let (device, queue) = adapter
                .request_device(
                    &DeviceDescriptor {
                        label: None,
                        features: Features::empty(),
                        limits: Limits::downlevel_webgl2_defaults()
                            .using_resolution(adapter.limits()),
                    },
                    None,
                )
                .await
                .unwrap();

            device.on_uncaptured_error(|e| panic!("WGPU error: {:?}", e));

            (surface_format, device, queue)
        });

        Self {
            surface,
            surface_format,
            device,
            queue,
        }
    }

    pub fn configure(&self, width: u32, height: u32) {
        self.surface.configure(
            &self.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: self.surface_format,
                width,
                height,
                present_mode: PresentMode::Fifo,
            },
        );
    }

    pub fn create_buffer<T: Pod>(&self, contents: &[T], usage: BufferUsages) -> TypedBuffer<T> {
        let inner = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(contents),
            usage,
        });

        let len = contents.len() as u64;

        TypedBuffer {
            inner,
            len,
            _p: PhantomData,
        }
    }

    pub fn create_line_pipeline(
        &self,
        uniform_buffer_layout: &UniformBufferLayout,
    ) -> LinePipeline {
        let module = self.device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(include_str!("line.wgsl").into()),
        });

        let inner = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(
                    &self
                        .device
                        .create_pipeline_layout(&PipelineLayoutDescriptor {
                            label: None,
                            bind_group_layouts: &[&uniform_buffer_layout.inner],
                            push_constant_ranges: &[],
                        }),
                ),
                vertex: VertexState {
                    module: &module,
                    entry_point: "main",
                    buffers: &[VertexBufferLayout {
                        array_stride: size_of::<LineVertex>() as u64,
                        step_mode: VertexStepMode::Vertex,
                        attributes: &vertex_attr_array![0 => Float32x3, 1 => Float32x4],
                    }],
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::LineList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: MSAA_SAMPLE_COUNT,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(FragmentState {
                    module: &module,
                    entry_point: "main",
                    targets: &[self.surface_format.into()],
                }),
            });

        LinePipeline { inner }
    }

    pub fn create_brush_pipeline(
        &self,
        uniform_buffer_layout: &UniformBufferLayout,
        texture_layout: &TextureLayout,
    ) -> BrushPipeline {
        let module = self.device.create_shader_module(&ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(include_str!("brush.wgsl").into()),
        });

        let inner = self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(
                    &self
                        .device
                        .create_pipeline_layout(&PipelineLayoutDescriptor {
                            label: None,
                            bind_group_layouts: &[
                                &uniform_buffer_layout.inner,
                                &uniform_buffer_layout.inner,
                                &texture_layout.inner,
                            ],
                            push_constant_ranges: &[],
                        }),
                ),
                vertex: VertexState {
                    module: &module,
                    entry_point: "main",
                    buffers: &[VertexBufferLayout {
                        array_stride: size_of::<BrushVertex>() as u64,
                        step_mode: VertexStepMode::Vertex,
                        attributes: &vertex_attr_array![
                            0 => Float32x3,
                            1 => Float32x3,
                            2 => Float32x2,
                        ],
                    }],
                },
                primitive: PrimitiveState {
                    cull_mode: Some(Face::Back),
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: MSAA_SAMPLE_COUNT,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(FragmentState {
                    module: &module,
                    entry_point: "main",
                    targets: &[self.surface_format.into()],
                }),
            });

        BrushPipeline { inner }
    }

    pub fn begin_frame(&self) -> Frame {
        let texture = self.surface.get_current_texture().unwrap();
        let view = texture.texture.create_view(&Default::default());
        let encoder = self.device.create_command_encoder(&Default::default());

        Frame {
            texture,
            view,
            encoder,
        }
    }

    pub fn end_frame(&self, frame: Frame) {
        self.queue.submit(once(frame.encoder.finish()));
        frame.texture.present();
    }

    pub fn create_uniform_buffer_layout(&self) -> UniformBufferLayout {
        let inner = self
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        UniformBufferLayout { inner }
    }

    pub fn create_uniform_buffer_group<T: Pod>(
        &self,
        layout: &UniformBufferLayout,
        content: T,
    ) -> UniformBufferGroup<T> {
        let buffer = self.create_buffer(
            cast_slice(&[content]),
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );

        let inner = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout.inner,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer.inner,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        UniformBufferGroup { inner, buffer }
    }

    pub fn upload_uniform<T: Pod>(&self, group: &UniformBufferGroup<T>, content: T) {
        self.queue
            .write_buffer(&group.buffer.inner, 0, cast_slice(&[content]));
    }

    pub fn create_texture_layout(&self) -> TextureLayout {
        TextureLayout {
            inner: self
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Texture {
                                sample_type: TextureSampleType::Float { filterable: false },
                                view_dimension: TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            ty: BindingType::Sampler {
                                filtering: true,
                                comparison: false,
                            },
                            count: None,
                        },
                    ],
                }),
        }
    }

    pub fn create_texture_group(
        &self,
        layout: &TextureLayout,
        image: &DynamicImage,
        sampler: &Sampler,
    ) -> TextureGroup {
        let size = {
            let (width, height) = image.dimensions();
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            }
        };

        let texture = self.device.create_texture(&TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
        });

        let view = texture.create_view(&Default::default());

        let inner = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout.inner,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler),
                },
            ],
        });

        self.queue.write_texture(
            ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            image.as_rgba8().unwrap().as_bytes(),
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(size.width * 4),
                rows_per_image: NonZeroU32::new(size.height),
            },
            size,
        );

        TextureGroup { inner }
    }

    pub fn create_msaa_framebuffer(&self, width: u32, height: u32) -> MsaaFramebuffer {
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let view = self
            .device
            .create_texture(&TextureDescriptor {
                label: None,
                size,
                mip_level_count: 1,
                sample_count: MSAA_SAMPLE_COUNT,
                dimension: TextureDimension::D2,
                format: self.surface_format,
                usage: TextureUsages::RENDER_ATTACHMENT,
            })
            .create_view(&Default::default());

        MsaaFramebuffer { view }
    }

    pub fn create_sampler(&self) -> Sampler {
        self.device.create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            ..Default::default()
        })
    }
}

impl Frame {
    pub fn begin_pass<'a>(&'a mut self, color: [f64; 4], msaa: &'a MsaaFramebuffer) -> Pass {
        Pass {
            inner: self.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &msaa.view,
                    resolve_target: Some(&self.view),
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: color[0],
                            g: color[1],
                            b: color[2],
                            a: color[3],
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            }),
        }
    }
}

impl<'a> Pass<'a> {
    pub fn set_camera_group(&mut self, camera_group: &'a UniformBufferGroup<CameraBlock>) {
        self.inner.set_bind_group(0, &camera_group.inner, &[]);
    }

    pub fn set_transform(&mut self, transform_group: &'a UniformBufferGroup<TransformBlock>) {
        self.inner.set_bind_group(1, &transform_group.inner, &[]);
    }

    pub fn set_texture(&mut self, texture_group: &'a TextureGroup) {
        self.inner.set_bind_group(2, &texture_group.inner, &[]);
    }

    pub fn begin_lines(&mut self, pipeline: &'a LinePipeline) {
        self.inner.set_pipeline(&pipeline.inner);
    }

    pub fn draw_lines(&mut self, buffer: &'a TypedBuffer<LineVertex>) {
        self.inner.set_vertex_buffer(0, buffer.inner.slice(..));
        self.inner.draw(0..buffer.len as u32, 0..1);
    }

    pub fn begin_brushes(&mut self, pipeline: &'a BrushPipeline) {
        self.inner.set_pipeline(&pipeline.inner);
    }

    pub fn draw_mesh<V: Pod>(
        &mut self,
        vertices: &'a TypedBuffer<V>,
        triangles: &'a TypedBuffer<Triangle>,
    ) {
        self.inner.set_vertex_buffer(0, vertices.inner.slice(..));
        self.inner
            .set_index_buffer(triangles.inner.slice(..), IndexFormat::Uint16);
        self.inner
            .draw_indexed(0..triangles.len as u32 * 3, 0, 0..1);
    }
}
