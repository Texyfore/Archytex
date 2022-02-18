mod camera;
mod move_tool;
mod new_solid;
mod rotate_tool;

use crate::{
    data::PropInfoContainer,
    graphics::{Canvas, Graphics},
    logic::{camera::Camera, elements::ElementKind, input::Input, scene::Scene},
};

use asset::{PropID, TextureID};
pub use camera::CameraTool;
pub use new_solid::NewSolid;

pub trait Tool {
    fn process(&mut self, _ctx: Context) -> Option<Box<dyn Tool>> {
        None
    }

    fn render(&self, _canvas: &mut Canvas) {}

    fn can_switch(&self) -> bool {
        false
    }
}

pub struct Context<'a> {
    pub input: &'a Input,
    pub graphics: &'a Graphics,
    pub prop_infos: &'a PropInfoContainer,
    pub camera: &'a mut Camera,
    pub scene: &'a mut Scene,
    pub delta: f32,
    pub mode: ElementKind,
    pub grid: &'a mut i32,
    pub texture: TextureID,
    pub prop: PropID,
}
