use anyhow::Result;
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::ipc::IpcHost;

pub struct MainLoop;

impl MainLoop {
    pub fn new(_window: &Window) -> Result<Self> {
        Ok(Self)
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<()> {
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        Ok(())
    }

    pub fn keyboard_input(&mut self, _key: VirtualKeyCode, _state: ElementState) -> Result<()> {
        Ok(())
    }

    pub fn mouse_input(&mut self, _button: MouseButton, _state: ElementState) -> Result<()> {
        Ok(())
    }
}
