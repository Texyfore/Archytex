mod camera;
mod element;
mod input;
mod tools;

use cgmath::vec2;
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    graphics::{Canvas, Graphics},
    OnSave,
};

use self::{camera::Camera, element::ElementKind, input::Input, tools::ToolHub};

pub struct Logic {
    input: Input,
    camera: Camera,
    tool_hub: ToolHub,
}

impl Logic {
    pub fn init(ctx: Context) -> Self {
        Self {
            input: Input::default(),
            camera: Camera::default(),
            tool_hub: ToolHub::init(tools::Context { up: &ctx }),
        }
    }

    pub fn process(&mut self, ctx: Context) {
        self.tool_hub.process(tools::Context { up: &ctx });

        for (key, kind) in [
            (VirtualKeyCode::Key1, ElementKind::Solid),
            (VirtualKeyCode::Key2, ElementKind::Face),
            (VirtualKeyCode::Key3, ElementKind::Point),
            (VirtualKeyCode::Key4, ElementKind::Prop),
        ] {
            if self.input.is_key_down_once(key) {
                self.tool_hub.change_logic(kind);
            }
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
        canvas.set_camera_matrices(self.camera.matrices());
    }
}

pub struct Context<'a> {
    pub graphics: &'a Graphics,
    pub save_handler: &'a dyn OnSave,
    pub delta: f32,
}
