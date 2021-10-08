use crate::{
    api::{
        fragment_collector::FragmentCollector,
        fragment_render::{FragmentContext, FragmentRender},
    },
    utilities::math::Vec3,
    vector,
};

pub struct ArrayCollector {}

impl<T: FragmentRender> FragmentCollector<T> for ArrayCollector {
    type Output = Vec<Vec<Vec3>>;
    fn collect(&self, fragment_render: T, width: usize, height: usize) -> Self::Output {
        let ctx = FragmentContext {
            width: width as _,
            height: height as _,
        };
        let mut rows = Vec::with_capacity(height);
        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            let y = y as f64 / ctx.height;
            for x in 0..width {
                let x = x as f64 / ctx.width;
                let fragment = fragment_render.render_fragment(&ctx, vector!(x, y));
                row.push(fragment);
            }
            rows.push(row);
        }
        rows
    }
}
