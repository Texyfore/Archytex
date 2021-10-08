use crate::utilities::math::Vec3;

use super::fragment_render::FragmentRender;

pub trait FragmentCollector<T: FragmentRender> {
    type Output;
    fn collect(&self, fragment_render: T, width: usize, height: usize) -> Self::Output;
}
