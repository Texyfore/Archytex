use crate::{api::fragment_render::{FragmentRender, FragmentContext}, utilities::math::{Vec2, Vec3}};

pub struct SamplingRenderer<Renderer: FragmentRender + Sync + Send> {
    pub inner: Renderer,
    pub samples: usize,
}

impl<Renderer: FragmentRender + Sync + Send> FragmentRender for SamplingRenderer<Renderer> {
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        // (0..self.samples)
        //     .into_par_iter()
        //     .map(|_| self.inner.render_fragment(ctx, pos))
        //     .reduce(Vec3::default, |a, b| a + b)
        //     / (self.samples as f64)
        (0..self.samples)
            .into_iter()
            .map(|_| self.inner.render_fragment(ctx, pos))
            .fold(Vec3::default(), |a, b| a + b)
            / (self.samples as f64)
    }
}
