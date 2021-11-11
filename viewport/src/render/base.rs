use futures_lite::future;
use wgpu::*;
use winit::window::Window;

pub struct Context {
    pub(super) surface: Surface,
    pub(super) surface_format: TextureFormat,
    pub(super) device: Device,
    pub(super) queue: Queue,
}

impl Context {
    pub(super) fn new(window: &Window) -> Self {
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
}
