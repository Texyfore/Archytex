use anyhow::Result;
use renderer::{scene::Scene, Renderer};
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::ipc::IpcHost;

pub struct MainLoop {
    renderer: Renderer,
}

impl MainLoop {
    pub fn new(window: &Window) -> Result<Self> {
        Ok(Self {
            renderer: Renderer::new(window)?,
        })
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<()> {
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        self.renderer.render(&mut Scene)?;
        Ok(())
    }

    pub fn window_resized(&self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    pub fn keyboard_input(&mut self, _key: VirtualKeyCode, _state: ElementState) -> Result<()> {
        Ok(())
    }

    pub fn mouse_input(&mut self, _button: MouseButton, _state: ElementState) -> Result<()> {
        Ok(())
    }
}
