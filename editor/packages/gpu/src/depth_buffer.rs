use wgpu::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
};

use crate::{Gpu, MSAA_SAMPLE_COUNT};

pub struct DepthBuffer {
    pub(crate) view: TextureView,
}

impl Gpu {
    pub fn create_depth_buffer(&self, width: u32, height: u32) -> DepthBuffer {
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: MSAA_SAMPLE_COUNT,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
        });

        let view = texture.create_view(&Default::default());

        DepthBuffer { view }
    }
}
