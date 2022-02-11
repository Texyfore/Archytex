mod camera;
mod element;
mod input;
mod scene;

use cgmath::vec2;
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    graphics::{Canvas, Graphics},
    Host,
};

use self::{camera::Camera, input::Input};

pub struct Logic {
    input: Input,
    camera: Camera,
}

impl Logic {
    pub fn init(_ctx: Context) -> Self {
        let input = Input::default();
        let camera = Camera::default();

        Self { input, camera }
    }

    pub fn process(&mut self, _ctx: Context) {
        self.input.process();
    }

    pub fn resized(&mut self, width: u32, height: u32) {
        self.camera.recalc(width, height);
    }

    pub fn key(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.input.key(key, state);
    }

    pub fn button(&mut self, button: MouseButton, state: ElementState) {
        self.input.button(button, state);
    }

    pub fn movement(&mut self, x: f32, y: f32) {
        self.input.movement(vec2(x, y));
    }

    pub fn scroll(&mut self, delta: f32) {
        self.input.scroll(delta);
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.set_camera_matrices(self.camera.matrices());
    }
}

pub struct Context<'h, 'g> {
    pub host: &'h dyn Host,
    pub graphics: &'g Graphics,
    pub delta: f32,
}
