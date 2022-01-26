use crate::editor::graphics::GraphicsMask;

use super::{Context, Tool};

#[derive(Default)]
pub struct Select;

impl Tool for Select {
    fn process(&mut self, _ctx: &mut Context) {}

    fn cancelled(&mut self, _ctx: &mut Context) {}

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}
