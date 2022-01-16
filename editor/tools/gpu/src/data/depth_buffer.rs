use wgpu::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
};

use crate::handle::GpuHandle;

pub struct DepthBuffer {
    pub(crate) view: TextureView,
}

impl GpuHandle {
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
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
        });

        let view = texture.create_view(&Default::default());

        DepthBuffer { view }
    }
}
