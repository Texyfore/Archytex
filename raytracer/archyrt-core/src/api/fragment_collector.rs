use crate::textures::texture_repo::TextureRepository;

use super::fragment_render::FragmentRender;

pub trait FragmentCollector<T: FragmentRender> {
    type Output;
    fn collect(
        &self,
        fragment_render: T,
        texture_repo: &TextureRepository,
        width: usize,
        height: usize,
    ) -> Self::Output;
}
