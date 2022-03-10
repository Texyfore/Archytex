use winit::event::VirtualKeyCode;

use crate::logic::{
    camera::Camera,
    common::Axis,
    scene::{self, Action},
};

use super::{Context, Tool};

#[derive(Default)]
pub struct RotateSolid;

impl Tool for RotateSolid {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        for (key, axis) in [
            (VirtualKeyCode::X, Axis::X),
            (VirtualKeyCode::Y, Axis::Y),
            (VirtualKeyCode::Z, Axis::Z),
        ] {
            if ctx.input.is_key_down_once(key) {
                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::RotateSolids(axis, false),
                );
                break;
            }
        }

        if ctx.input.is_key_down_once(VirtualKeyCode::Escape) {
            return Some(Box::new(Camera::default()));
        }

        None
    }
}
