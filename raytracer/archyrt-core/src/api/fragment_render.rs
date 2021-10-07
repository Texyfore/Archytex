use crate::utilities::math::{Vec2, Vec3};

pub struct FragmentContext{
    pub width: f64,
    pub height: f64,
}

pub trait FragmentRender{
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3;
}