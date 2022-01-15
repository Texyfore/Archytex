use std::iter::once;

use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    SurfaceTexture, TextureView,
};

use crate::handle::GpuHandle;

pub struct Frame {
    pub(crate) texture: SurfaceTexture,
    pub(crate) view: TextureView,
    pub(crate) encoder: CommandEncoder,
}

impl Frame {
    pub fn begin_pass(&mut self, clear_color: [f32; 3]) -> RenderPass {
        RenderPass {
            pass: self.encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &self.view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: clear_color[0] as f64,
                            g: clear_color[1] as f64,
                            b: clear_color[2] as f64,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            }),
        }
    }

    pub fn draw(self, gpu: &GpuHandle) {
        gpu.queue.submit(once(self.encoder.finish()));
        self.texture.present();
    }
}

pub struct RenderPass<'a> {
    pass: wgpu::RenderPass<'a>,
}
