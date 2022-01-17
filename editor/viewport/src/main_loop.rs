use std::time::Instant;

use anyhow::Result;
use cgmath::Vector2;
use renderer::Renderer;
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::{
    editor::{self, Editor},
    input::Input,
    ipc::IpcHost,
};

pub struct MainLoop {
    renderer: Renderer,
    input: Input,
    editor: Editor,
    before: Instant,
}

impl MainLoop {
    pub fn new(window: &Window) -> Result<Self> {
        let mut renderer = Renderer::new(window)?;
        let input = Input::default();
        let editor = Editor::default();

        {
            let (width, height) = window.inner_size().into();
            renderer.resize(width, height);
        }

        Ok(Self {
            renderer,
            input,
            editor,
            before: Instant::now(),
        })
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<()> {
        let after = Instant::now();
        let delta = (after - self.before).as_secs_f32();
        self.before = after;

        self.editor.process(editor::OuterContext {
            delta,
            input: &self.input,
        })?;
        self.input.process();
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        self.editor.render(&self.renderer)?;
        Ok(())
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.editor.window_resized(width, height);
        self.renderer.resize(width, height);
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.input.keyboard_input(key, state);
    }

    pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.input.mouse_input(button, state);
    }

    pub fn mouse_movement(&mut self, new_pos: Vector2<f32>) {
        self.input.mouse_movement(new_pos);
    }

    pub fn mouse_wheel_movement(&mut self, movement: f32) {
        self.input.mouse_wheel_movement(movement);
    }
}
