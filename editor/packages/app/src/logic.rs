use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::graphics::{Canvas, Renderer};

pub struct Logic;

impl Logic {
    pub fn init(_ctx: Context) -> Self {
        Self
    }

    pub fn process(&mut self, _ctx: Context) {}

    pub fn resized(&mut self, _width: u32, _height: u32) {}

    pub fn key(&mut self, _code: VirtualKeyCode, _state: ElementState) {}

    pub fn button(&mut self, _button: MouseButton, _state: ElementState) {}

    pub fn movement(&mut self, _x: f32, _y: f32) {}

    pub fn scroll(&mut self, _delta: f32) {}

    pub fn render(&self, _canvas: &mut Canvas) {}
}

pub struct Context<'a> {
    pub renderer: &'a Renderer,
}
