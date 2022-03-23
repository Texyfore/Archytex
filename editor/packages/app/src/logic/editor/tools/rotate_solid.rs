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
            (VirtualKeyCode::Z, Axis::Y),
            (VirtualKeyCode::Y, Axis::Z),
        ] {
            if ctx.input.is_key_down_once(key) {
                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::RotateSolids {
                        axis,
                        iters: 1,
                        reverse: ctx.input.is_key_down(VirtualKeyCode::LShift),
                        snap: *ctx.grid / 2,
                    },
                );

                return Some(Box::new(CameraTool::new(ctx.graphics)));
            }
        }

        None
    }
}
