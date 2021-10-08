use crate::utilities::math::Vec3;

use super::fragment_render::FragmentRender;

pub trait FragmentCollector<T: FragmentRender> {
    fn collect(&self, fragment_render: T, width: usize, height: usize) -> Vec<Vec<Vec3>>;
}
