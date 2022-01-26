use cgmath::vec3;

use crate::editor::{
    graphics::GraphicsMask,
    scene::{Action, Solid},
};

use super::{Context, Tool};

#[derive(Default)]
pub struct Select;

impl Tool for Select {
    fn process(&mut self, ctx: &mut Context) {
        if ctx
            .input()
            .is_button_down_once(winit::event::MouseButton::Left)
        {
            ctx.scene().act(Action::AddSolid(Solid::new(
                vec3(0, 0, 0),
                vec3(100, 100, 100),
            )));
            ctx.set_regen();
        }

        self.process_camera(ctx);
        self.process_undo_redo(ctx);
    }

    fn cancelled(&mut self, _ctx: &mut Context) {}

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}
