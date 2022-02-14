mod camera;

use crate::{
    graphics::{Canvas, Graphics},
    logic::{camera::Camera, elements::ElementKind, input::Input, scene::Scene},
};

pub use camera::CameraTool;

pub trait Tool {
    fn process(&mut self, _ctx: Context) {}

    fn render(&self, _canvas: &mut Canvas) {}
}

pub struct Context<'a> {
    pub input: &'a Input,
    pub graphics: &'a Graphics,
    pub camera: &'a mut Camera,
    pub scene: &'a mut Scene,
    pub delta: f32,
    pub mode: ElementKind,
}
