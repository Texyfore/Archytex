use crate::{
    api::fragment_render::{FragmentContext, FragmentRender},
    utilities::math::{Vec2, Vec3},
};

pub struct BasicRenderer {}

impl FragmentRender for BasicRenderer {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        todo!()
    }
}
