mod input;

use cgmath::vec2;
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    graphics::{Canvas, Renderer},
    OnSave,
};

use self::input::Input;

pub struct Logic {
    input: Input,
}

impl Logic {
    pub fn init(_ctx: Context) -> Self {
        Self {
            input: Input::default(),
        }
    }

    pub fn process(&mut self, _ctx: Context) {
        self.input.process();
    }

    pub fn resized(&mut self, _width: u32, _height: u32) {}

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

    pub fn render(&self, _canvas: &mut Canvas) {}
}

pub struct Context<'a> {
    pub renderer: &'a Renderer,
    pub save_handler: &'a dyn OnSave,
}
