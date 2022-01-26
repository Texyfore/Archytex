use crate::textures::texture_repo::TextureRepository;

use super::fragment_render::FragmentRender;

pub trait FragmentCollector<T: FragmentRender> {
    type Output;
    fn collect<R: TextureRepository + Sync>(
        &self,
        fragment_render: T,
        texture_repo: R,
        width: usize,
        height: usize,
    ) -> Self::Output;
}
