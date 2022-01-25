use crate::editor::scene::GraphicsMask;

use super::{Context, Tool};

#[derive(Default)]
pub struct Select;

impl Tool for Select {
    fn process(&mut self, ctx: &mut Context) {
        self.process_camera(ctx);
        self.process_undo_redo(ctx);
    }

    fn cancelled(&mut self, ctx: &mut Context) {}

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}
