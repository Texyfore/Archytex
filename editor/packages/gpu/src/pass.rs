use std::{iter::once, ops::Range};

use wgpu::{
    Color, CommandEncoder, IndexFormat, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, RenderPassDescriptor, SurfaceTexture, TextureView,
};

use crate::{Buffer, DepthBuffer, Gpu, Pipeline, Surface, Texture, Uniform};

pub struct Frame {
    texture: SurfaceTexture,
    view: TextureView,
    encoder: CommandEncoder,
}

pub struct RenderPass<'a> {
    pass: wgpu::RenderPass<'a>,
}

pub struct InstanceConfig<'a, I> {
    pub slot: u32,
    pub buffer: &'a Buffer<I>,
    pub range: Range<u32>,
}

impl Frame {
    pub fn begin_pass<'a>(
        &'a mut self,
        depth_buffer: &'a DepthBuffer,
        clear_color: &[f32; 3],
    ) -> RenderPass {
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
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &depth_buffer.view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            }),
        }
    }
}

impl Gpu {
    pub fn begin_frame(&self, surface: &Surface) -> Frame {
        let texture = surface.surface.get_current_texture().unwrap();
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
}

impl<'a> RenderPass<'a> {
    pub fn set_pipeline(&mut self, pipeline: &'a Pipeline) {
        self.pass.set_pipeline(&pipeline.pipeline);
    }

    pub fn set_uniform<T>(&mut self, index: u32, uniform: &'a Uniform<T>) {
        self.pass.set_bind_group(index, &uniform.group, &[]);
    }

    pub fn set_texture(&mut self, index: u32, texture: &'a Texture) {
        self.pass.set_bind_group(index, &texture.group, &[]);
    }

    pub fn draw<V>(&mut self, vertices: &'a Buffer<V>) {
        self.pass.set_vertex_buffer(0, vertices.buffer.slice(..));
        self.pass.draw(0..vertices.len() as u32, 0..1);
    }

    pub fn draw_triangles<V>(&mut self, vertices: &'a Buffer<V>, triangles: &'a Buffer<[u16; 3]>) {
        self.pass.set_vertex_buffer(0, vertices.buffer.slice(..));
        self.pass
            .set_index_buffer(triangles.buffer.slice(..), IndexFormat::Uint16);
        self.pass
            .draw_indexed(0..triangles.len() as u32 * 3, 0, 0..1);
    }

    pub fn draw_triangles_instanced<V, I>(
        &mut self,
        vertices: &'a Buffer<V>,
        triangles: &'a Buffer<[u16; 3]>,
        config: InstanceConfig<'a, I>,
    ) {
        self.pass.set_vertex_buffer(0, vertices.buffer.slice(..));
        self.pass
            .set_vertex_buffer(config.slot, config.buffer.buffer.slice(..));
        self.pass
            .set_index_buffer(triangles.buffer.slice(..), IndexFormat::Uint16);
        self.pass
            .draw_indexed(0..triangles.len() as u32 * 3, 0, config.range);
    }
}
