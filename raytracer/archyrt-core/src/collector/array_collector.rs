use crate::{
    api::{
        fragment_collector::FragmentCollector,
        fragment_render::{FragmentContext, FragmentRender},
    },
    textures::texture_repo::TextureRepository,
    utilities::math::Vec3,
    vector,
};

pub struct ArrayCollector {}

impl<T: FragmentRender> FragmentCollector<T> for ArrayCollector {
    type Output = Vec<Vec<Vec3>>;
    fn collect<R: TextureRepository + Sync>(
        &self,
        fragment_render: T,
        texture_repo: R,
        width: usize,
        height: usize,
    ) -> Self::Output {
        let ctx = FragmentContext {
            width: width as _,
            height: height as _,
            repo: texture_repo,
        };
        let mut rows = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            let y = y as f64 / (ctx.height - 1.0);
            for x in 0..width {
                let x = x as f64 / (ctx.width - 1.0);
                let fragment = fragment_render.render_fragment(&ctx, vector!(x, y));
                row.push(fragment);
            }
            rows.push(row);
        }
        rows
    }
}
