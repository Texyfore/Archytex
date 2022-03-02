use archyrt_core::{api::fragment_render::{FragmentRender, FragmentContext}, utilities::math::{Vec2, Vec3}, vector};

pub struct ShiftedView<T: FragmentRender>{
    pub inner: T,
    pub x: f64,
    pub y: f64,
    pub full_w: usize,
    pub full_h: usize
}

impl<T: FragmentRender> FragmentRender for ShiftedView<T>{
    fn render_fragment(&self, ctx: &FragmentContext, pos: Vec2) -> Vec3 {
        let newctx = FragmentContext{
            width: self.full_w as f64,
            height: self.full_h as f64,
            repo: ctx.repo,
        };
        self.inner.render_fragment(&newctx, pos*(vector![ctx.width, ctx.height]/vector![newctx.width, newctx.height])+vector![self.x, self.y])
    }
}