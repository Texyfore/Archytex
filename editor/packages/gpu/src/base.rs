use futures_lite::future;
use raw_window_handle::HasRawWindowHandle;
use wgpu::{
    Backends, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, Device, DeviceDescriptor, Features, Instance, Limits, PresentMode, Queue,
    RequestAdapterOptions, SamplerBindingType, ShaderStages, SurfaceConfiguration, TextureFormat,
    TextureSampleType, TextureUsages, TextureViewDimension,
};

pub struct Gpu {
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) texture_layout: BindGroupLayout,
    pub(crate) uniform_layout: BindGroupLayout,
}

pub struct Surface {
    pub(crate) surface: wgpu::Surface,
    pub(crate) format: TextureFormat,
}

impl Surface {
    pub fn configure(&self, gpu: &Gpu, width: u32, height: u32) {
        self.surface.configure(
            &gpu.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: self.format,
                width,
                height,
                present_mode: PresentMode::Fifo,
            },
        );
    }
}

pub fn init<H>(window: &H) -> (Gpu, Surface)
where
    H: HasRawWindowHandle,
{
    let instance = Instance::new(Backends::all());
    let surface = unsafe { instance.create_surface(window) };

    let (format, device, queue) = future::block_on(async {
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let format = surface
            .get_preferred_format(&adapter)
            .unwrap_or(TextureFormat::Bgra8UnormSrgb);

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::empty(),
                    limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .unwrap();

        (format, device, queue)
    });

    device.on_uncaptured_error(|e| panic!("---- GPU ERROR ----\n{}\n-------------------", e));

    let texture_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            },
        ],
    });

    let uniform_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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

    (
        Gpu {
            device,
            queue,
            texture_layout,
            uniform_layout,
        },
        Surface { surface, format },
    )
}
