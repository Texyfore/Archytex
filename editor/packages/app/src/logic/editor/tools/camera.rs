use winit::event::{MouseButton, VirtualKeyCode};

use super::{Context, Tool};

pub struct CameraTool;

impl Tool for CameraTool {
    fn process(&mut self, ctx: Context) {
        if !ctx.input.is_button_down(MouseButton::Right) {
            return;
        }

        if ctx.input.is_key_down(VirtualKeyCode::W) {
            ctx.camera.move_forward(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::S) {
            ctx.camera.move_backward(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::A) {
            ctx.camera.move_left(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::D) {
            ctx.camera.move_right(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::Q) {
            ctx.camera.move_down(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::E) {
            ctx.camera.move_up(ctx.delta);
        }

        if ctx.input.mouse_wheel().abs() > 0.1 {
            if ctx.input.mouse_wheel() > 0.0 {
                ctx.camera.increase_speed();
            } else {
                ctx.camera.decrease_speed();
            }
        }

        ctx.camera.look(ctx.input.mouse_delta(), ctx.delta);
    }
}
