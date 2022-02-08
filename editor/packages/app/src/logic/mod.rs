mod camera;
mod input;

use cgmath::vec2;
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    graphics::{Canvas, Renderer},
    OnSave,
};

use self::{camera::Camera, input::Input};

pub struct Logic {
    input: Input,
    camera: Camera,
}

impl Logic {
    pub fn init(_ctx: Context) -> Self {
        Self {
            input: Input::default(),
            camera: Camera::default(),
        }
    }

    pub fn process(&mut self, ctx: Context) {
        if self.input.is_key_down(VirtualKeyCode::W) {
            self.camera.move_forward(ctx.delta);
        }
        if self.input.is_key_down(VirtualKeyCode::S) {
            self.camera.move_backward(ctx.delta);
        }
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
        canvas.set_camera(self.camera.matrices());
    }
}

pub struct Context<'a> {
    pub renderer: &'a Renderer,
    pub save_handler: &'a dyn OnSave,
    pub delta: f32,
}
