use anyhow::Result;
use renderer::{scene::Scene, Renderer};
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::{input::Input, ipc::IpcHost};

pub struct MainLoop {
    renderer: Renderer,
    input: Input,
}

impl MainLoop {
    pub fn new(window: &Window) -> Result<Self> {
        Ok(Self {
            renderer: Renderer::new(window)?,
            input: Input::default(),
        })
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<()> {
        self.input.process();
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        self.renderer.render(&mut Scene)?;
        Ok(())
    }

    pub fn window_resized(&self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.input.keyboard_input(key, state);
    }

    pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.input.mouse_input(button, state);
    }
}
