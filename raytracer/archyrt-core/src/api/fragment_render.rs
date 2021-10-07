use crate::utilities::math::Vec3;

pub struct FragmentContext{
    pub width: f64,
    pub height: f64,
}

pub trait FragmentRender<T: Fn(f64, f64)->Vec3>{
    fn render_fragment(&self, ctx: &FragmentContext) -> T;
}