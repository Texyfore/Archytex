use std::{iter::once, marker::PhantomData, mem::size_of};

use bytemuck::{cast_slice, Pod};
use futures_lite::future;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    *,
};
use winit::window::Window;

use super::{data::LineVertex, CameraBlock};

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
                multisample: Default::default(),
                fragment: Some(FragmentState {
                    module: &module,
                    entry_point: "main",
                    targets: &[self.surface_format.into()],
                }),
            });

        LinePipeline { inner }
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
}

impl Frame {
    pub fn begin_pass(&mut self) -> Pass {
        Pass {
            inner: self.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &self.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.1,
                            a: 1.0,
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

    pub fn begin_lines(&mut self, pipeline: &'a LinePipeline) {
        self.inner.set_pipeline(&pipeline.inner);
    }

    pub fn draw_lines(&mut self, buffer: &'a TypedBuffer<LineVertex>) {
        self.inner.set_vertex_buffer(0, buffer.inner.slice(..));
        self.inner.draw(0..buffer.len as u32, 0..1);
    }
}
