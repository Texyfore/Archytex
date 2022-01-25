pub mod solid;

mod context;

use winit::event::{MouseButton, VirtualKeyCode};

pub use self::context::Context;

use super::scene::GraphicsMask;

pub trait Tool {
    fn process(&mut self, ctx: &mut Context);
    fn cancelled(&mut self, ctx: &mut Context);
    fn graphics_mask(&self) -> GraphicsMask;

    fn process_undo_redo(&mut self, ctx: &mut Context) {
        if ctx.input().is_key_down(VirtualKeyCode::LControl) {
            if ctx.input().is_key_down_once(VirtualKeyCode::Z) {
                ctx.scene().undo();
            } else if ctx.input().is_key_down_once(VirtualKeyCode::Y) {
                ctx.scene().redo();
            }
        }
    }

    fn process_camera(&mut self, ctx: &mut Context) {
        if !ctx.input().is_button_down(MouseButton::Right) {
            return;
        }

        let delta = ctx.delta();

        if ctx.input().is_key_down(VirtualKeyCode::W) {
            ctx.camera().move_forward(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::S) {
            ctx.camera().move_backward(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::A) {
            ctx.camera().move_left(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::D) {
            ctx.camera().move_right(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::Q) {
            ctx.camera().move_down(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::E) {
            ctx.camera().move_up(delta);
        }

        if ctx.input().mouse_wheel().abs() > 0.1 {
            if ctx.input().mouse_wheel().signum() > 0.0 {
                ctx.camera().increase_speed();
            } else {
                ctx.camera().decrease_speed();
            }
        }

        let mouse_delta = ctx.input().mouse_delta();
        ctx.camera().look(mouse_delta, delta);
    }
}
