use winit::event::{MouseButton, VirtualKeyCode};

use crate::logic::scene;

use super::{Context, Tool};

pub struct CameraTool;

impl Tool for CameraTool {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        let mut ctx = ctx;
        if ctx.input.is_button_down(MouseButton::Right) {
            control(&mut ctx);
        } else {
            undo_redo(&mut ctx);
        }

        None
    }
}

fn control(ctx: &mut Context) {
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

fn undo_redo(ctx: &mut Context) {
    if ctx.input.is_key_down(VirtualKeyCode::LControl) {
        if ctx.input.is_key_down_once(VirtualKeyCode::Z) {
            ctx.scene.undo(scene::Context {
                graphics: ctx.graphics,
            });
        } else if ctx.input.is_key_down_once(VirtualKeyCode::Y) {
            ctx.scene.redo(scene::Context {
                graphics: ctx.graphics,
            });
        }
    }
}
