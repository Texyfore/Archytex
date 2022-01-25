use winit::event::{MouseButton, VirtualKeyCode};

use crate::editor::scene::{Action, RaycastHit};

use super::Context;

pub trait Select {
    fn deselect_action(&self) -> Action;
    fn select_action(&self, hit: RaycastHit) -> Option<Action>;

    fn generic_process(&mut self, ctx: &mut Context) -> bool {
        if ctx.input().was_button_down_once(MouseButton::Left) {
            if !ctx.input().is_key_down(VirtualKeyCode::LShift) {
                ctx.scene().act(self.deselect_action());
                ctx.set_regen();
            }

            let mouse_pos = ctx.input().mouse_pos();
            let ray = ctx.camera().screen_ray(mouse_pos);
            if let Some(hit) = ctx.scene().raycast(&ray) {
                if let Some(action) = self.select_action(hit) {
                    ctx.scene().act(action);
                    ctx.set_regen();
                }
            }

            true
        } else {
            false
        }
    }
}
