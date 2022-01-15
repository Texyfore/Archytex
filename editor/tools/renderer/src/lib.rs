pub mod scene;

mod gpu;

use raw_window_handle::HasRawWindowHandle;
use thiserror::Error;

use self::{gpu::GpuHandle, scene::Scene};

pub struct Renderer {
    gpu: GpuHandle,
}

impl Renderer {
    pub fn new<H: HasRawWindowHandle>(window_handle: &H) -> Result<Self, InitError> {
        let gpu = GpuHandle::new(window_handle)?;
        gpu.configure(1024, 768);
        Ok(Self { gpu })
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.gpu.configure(width, height);
    }

    pub fn render(&self, _scene: &mut Scene) -> Result<(), RenderError> {
        let mut frame = self.gpu.next_frame()?;

        {
            let _pass = frame.begin_pass([0.5; 3]);
        }

        frame.draw(&self.gpu.queue);

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error("Couldn't initialize renderer: {0}")]
    GpuError(#[from] gpu::InitError),
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Couldn't render frame: {0}")]
    NoNextFrame(#[from] gpu::NextFrameError),
}
