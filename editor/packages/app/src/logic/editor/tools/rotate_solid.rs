use winit::event::VirtualKeyCode;

use crate::logic::{
    common::Axis,
    scene::{self, Action},
};

use super::{CameraTool, Context, Tool};

pub struct RotateSolid;

impl Tool for RotateSolid {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        for (key, axis) in [
            (VirtualKeyCode::X, Axis::X),
            (VirtualKeyCode::Y, Axis::Y),
            (VirtualKeyCode::Z, Axis::Z),
        ] {
            if ctx.input.is_key_down_once(key) {
                let iters = if ctx.input.is_key_down(VirtualKeyCode::LShift) {
                    2
                } else {
                    1
                };

                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::RotateSolids {
                        axis,
                        iters,
                        reverse: false,
                    },
                );

                return Some(Box::new(CameraTool::new(ctx.graphics)));
            }
        }

        None
    }
}