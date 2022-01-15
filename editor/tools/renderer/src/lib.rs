pub mod scene;

use gpu::handle::GpuHandle;
use raw_window_handle::HasRawWindowHandle;
use thiserror::Error;

use self::scene::Scene;

pub struct Renderer {
    gpu: GpuHandle,
}

impl Renderer {
    pub fn new<H: HasRawWindowHandle>(window_handle: &H) -> Result<Self, NewError> {
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

        frame.draw(&self.gpu);

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum NewError {
    #[error("Couldn't create renderer: {0}")]
    GpuError(#[from] gpu::handle::NewError),
}

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Couldn't render frame: {0}")]
    NoNextFrame(#[from] gpu::handle::NextFrameError),
}
