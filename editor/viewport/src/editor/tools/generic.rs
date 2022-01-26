use std::marker::PhantomData;

use winit::event::VirtualKeyCode;

use crate::editor::{
    graphics::GraphicsMask,
    scene::{Action, RaycastHit},
};

use super::{Context, Tool};

#[derive(Default)]
pub struct Select<P: SelectProvider> {
    _p: PhantomData<P>,
}

impl<P: SelectProvider> Tool for Select<P> {
    fn process(&mut self, ctx: &mut Context) {
        if !ctx.input().is_key_down(VirtualKeyCode::LShift) {
            ctx.scene().act(P::deselect_action());
            ctx.set_regen();
        }

        let mouse_pos = ctx.input().mouse_pos();
        let ray = ctx.camera().screen_ray(mouse_pos);

        if let Some(hit) = ctx.scene().raycast(&ray) {
            if let Some(action) = P::select_action(hit) {
                ctx.scene().act(action);
                ctx.set_regen();
            }
        }

        ctx.switch_to(P::parent_tool());
    }

    fn graphics_mask(&self) -> GraphicsMask {
        P::graphics_mask()
    }
}

pub trait SelectProvider {
    fn deselect_action() -> Action;
    fn select_action(hit: RaycastHit) -> Option<Action>;
    fn parent_tool() -> Box<dyn Tool>;
    fn graphics_mask() -> GraphicsMask;
}
