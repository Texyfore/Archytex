use crate::{
    textures::texture_repo::TextureRepository,
    utilities::math::{Vec2, Vec3},
};

pub struct FragmentContext<R: TextureRepository> {
    pub width: f64,
    pub height: f64,
    pub repo: R,
}

pub trait FragmentRender {
    fn render_fragment<R: TextureRepository + Sync>(&self, ctx: &FragmentContext<R>, pos: Vec2) -> Vec3;
}

impl<T: FragmentRender> FragmentRender for &T{
    fn render_fragment<R: TextureRepository + Sync>(&self, ctx: &FragmentContext<R>, pos: Vec2) -> Vec3 {
        (*self).render_fragment(ctx, pos)
    }
}