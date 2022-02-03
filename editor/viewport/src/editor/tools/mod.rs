pub mod face;
pub mod point;
pub mod solid;
pub mod prop;

mod context;
mod generic;

use renderer::scene::Scene;
use winit::event::{MouseButton, VirtualKeyCode};

pub use self::context::Context;

use super::elements::ElementKind;

pub trait Tool {
    fn process(&mut self, ctx: &mut Context);
    fn render(&self, _scene: &mut Scene) {}
    fn element_mask(&self) -> ElementKind;

    fn cancellable(&self) -> bool {
        false
    }

    fn process_undo_redo(&mut self, ctx: &mut Context) {
        if ctx.input().is_key_down(VirtualKeyCode::LControl) {
            if ctx.input().is_key_down_once(VirtualKeyCode::Z) {
                ctx.scene_mut().undo();
                ctx.set_regen();
            } else if ctx.input().is_key_down_once(VirtualKeyCode::Y) {
                ctx.scene_mut().redo();
                ctx.set_regen();
            }
        }
    }

    fn process_camera(&mut self, ctx: &mut Context) {
        if !ctx.input().is_button_down(MouseButton::Right) {
            return;
        }

        let delta = ctx.delta();

        if ctx.input().is_key_down(VirtualKeyCode::W) {
            ctx.camera_mut().move_forward(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::S) {
            ctx.camera_mut().move_backward(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::A) {
            ctx.camera_mut().move_left(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::D) {
            ctx.camera_mut().move_right(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::Q) {
            ctx.camera_mut().move_down(delta);
        }

        if ctx.input().is_key_down(VirtualKeyCode::E) {
            ctx.camera_mut().move_up(delta);
        }

        if ctx.input().mouse_wheel().abs() > 0.1 {
            if ctx.input().mouse_wheel().signum() > 0.0 {
                ctx.camera_mut().increase_speed();
            } else {
                ctx.camera_mut().decrease_speed();
            }
        }

        let mouse_delta = ctx.input().mouse_delta();
        ctx.camera_mut().look(mouse_delta, delta);
    }
}
