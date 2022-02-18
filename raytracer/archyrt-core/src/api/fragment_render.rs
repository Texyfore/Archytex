use crate::{
    textures::texture_repo::TextureRepository,
    utilities::math::{Vec2, Vec3},
};

pub struct FragmentContext<'a> {
    pub width: f64,
    pub height: f64,
    pub repo: &'a TextureRepository,
}

pub trait FragmentRender {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3;
}

impl<T: FragmentRender> FragmentRender for &T {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        (*self).render_fragment(ctx, pos)
    }
}
