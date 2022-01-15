use renderer::{scene::Scene, Renderer};
use thiserror::Error;
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
    pub fn new(window: &Window) -> Result<Self, NewError> {
        Ok(Self {
            renderer: Renderer::new(window)?,
            input: Input::default(),
        })
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<(), ProcessError> {
        self.input.process();
        Ok(())
    }

    pub fn render(&self) -> Result<(), RenderError> {
        self.renderer.render(&mut Scene::default())?;
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

#[derive(Error, Debug)]
pub enum NewError {
    #[error("Couldn't create main loop: {0}")]
    NoRenderer(#[from] renderer::NewError),
}

#[derive(Error, Debug)]
#[error("Couldn't compute next frame")]
pub struct ProcessError;

#[derive(Error, Debug)]
#[error("{0}")]
pub struct RenderError(#[from] renderer::RenderError);
