use wgpu::{Extent3d, TextureDescriptor, TextureDimension, TextureUsages, TextureView};

use crate::{Gpu, Surface};

pub const MSAA_SAMPLE_COUNT: u32 = 4;

pub struct MsaaFramebuffer {
    pub(super) view: TextureView,
}

impl Gpu {
    pub fn create_msaa_framebuffer(
        &self,
        surface: &Surface,
        width: u32,
        height: u32,
    ) -> MsaaFramebuffer {
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
                format: surface.format,
                usage: TextureUsages::RENDER_ATTACHMENT,
            })
            .create_view(&Default::default());

        MsaaFramebuffer { view }
    }
}
