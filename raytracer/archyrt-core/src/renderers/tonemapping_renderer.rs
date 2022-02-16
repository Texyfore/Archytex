use crate::{
    api::fragment_render::{FragmentContext, FragmentRender},
    textures::texture_repo::TextureRepository,
    utilities::math::{Vec2, Vec3},
};

pub struct TonemappingRenderer<Renderer: FragmentRender> {
    pub inner: Renderer,
}

impl<Renderer: FragmentRender> FragmentRender for TonemappingRenderer<Renderer> {
    fn render_fragment<R: TextureRepository + Sync>(
        &self,
        ctx: &FragmentContext<R>,
        pos: Vec2,
    ) -> Vec3 {
        let c = self.inner.render_fragment(ctx, pos);
        c.to_srgb()
    }
}
