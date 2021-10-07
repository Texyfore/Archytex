use crate::api::fragment_render::FragmentRender;


pub struct BasicRenderer{

}

impl FragmentRender for BasicRenderer{
    fn render_fragment(&self, ctx: &crate::api::fragment_render::FragmentContext, pos: crate::utilities::math::Vec2) -> crate::utilities::math::Vec3 {
        todo!()
    }
}