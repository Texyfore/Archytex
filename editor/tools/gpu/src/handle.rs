use futures_lite::future;
use raw_window_handle::HasRawWindowHandle;
use thiserror::Error;
use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Limits, PresentMode, Queue,
    RequestAdapterOptions, RequestDeviceError, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};

pub struct GpuHandle {
    pub(crate) surface: Surface,
    pub(crate) surface_format: TextureFormat,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
}

impl GpuHandle {
    pub fn new<H: HasRawWindowHandle>(window_handle: &H) -> Result<Self, NewError> {
        let instance = wgpu::Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window_handle) };

        let (surface_format, device, queue) = future::block_on(async {
            let adapter = instance
                .request_adapter(&RequestAdapterOptions {
                    compatible_surface: Some(&surface),
                    ..Default::default()
                })
                .await
                .ok_or(NewError::NoAdapter)?;

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
                .await?;

            Result::<_, NewError>::Ok((surface_format, device, queue))
        })?;

        device.on_uncaptured_error(|e| panic!("{}", e));

        Ok(Self {
            surface,
            surface_format,
            device,
            queue,
        })
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
        )
    }
}

#[derive(Error, Debug)]
pub enum NewError {
    #[error("Couldn't select appropriate GPU")]
    NoAdapter,
    #[error("Selected GPU is unusable: {0}")]
    NoDevice(#[from] RequestDeviceError),
}
