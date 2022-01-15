use std::iter::once;

use thiserror::Error;
use wgpu::{
    Color, CommandEncoder, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    SurfaceError, SurfaceTexture, TextureView,
};

use crate::handle::GpuHandle;

pub struct Frame {
    texture: SurfaceTexture,
    view: TextureView,
    encoder: CommandEncoder,
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

impl GpuHandle {
    pub fn next_frame(&self) -> Result<Frame, NextFrameError> {
        let texture = self.surface.get_current_texture()?;
        let view = texture.texture.create_view(&Default::default());
        let encoder = self.device.create_command_encoder(&Default::default());

        Ok(Frame {
            texture,
            view,
            encoder,
        })
    }
}

#[derive(Error, Debug)]
#[error("Couldn't get next frame: {0}")]
pub struct NextFrameError(#[from] SurfaceError);

pub struct RenderPass<'a> {
    pass: wgpu::RenderPass<'a>,
}
