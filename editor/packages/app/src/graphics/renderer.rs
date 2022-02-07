use gpu::{DepthBuffer, Gpu, Surface};
use winit::window::Window;

use super::Canvas;

pub struct Renderer {
    gpu: Gpu,
    surface: Surface,
    depth_buffer: DepthBuffer,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let (width, height) = window.inner_size().into();

        let (gpu, surface) = gpu::init(window);
        let depth_buffer = gpu.create_depth_buffer(width, height);

        surface.configure(&gpu, width, height);

        Self {
            gpu,
            surface,
            depth_buffer,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface.configure(&self.gpu, width, height);
        self.depth_buffer = self.gpu.create_depth_buffer(width, height);
    }

    pub fn render(&self, _canvas: Canvas) {
        let mut frame = self.gpu.begin_frame(&self.surface);

        {
            let _pass = frame.begin_pass(&self.depth_buffer, &[0.1; 3]);
        }

        self.gpu.end_frame(frame);
    }
}
