mod base;

use base::Context;
use std::iter::once;
use wgpu::{
    Color, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    SurfaceConfiguration, TextureUsages,
};
use winit::window::Window;

pub struct Renderer {
    context: Context,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        Self {
            context: Context::new(window),
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.context.surface.configure(
            &self.context.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: self.context.surface_format,
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
            },
        );
    }

    pub fn render(&self) {
        let frame = self.context.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self
            .context
            .device
            .create_command_encoder(&Default::default());

        {
            let _pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        self.context.queue.submit(once(encoder.finish()));
        frame.present();
    }
}
