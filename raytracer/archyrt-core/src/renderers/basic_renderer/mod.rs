use crate::{
    api::fragment_render::{FragmentContext, FragmentRender},
    utilities::math::{Vec2, Vec3},
    vector,
};

pub struct BasicRenderer {}

impl Default for BasicRenderer {
    fn default() -> Self {
        Self {}
    }
}

impl FragmentRender for BasicRenderer {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        vector!(0.25, 0.5, 1.0)
    }
}
