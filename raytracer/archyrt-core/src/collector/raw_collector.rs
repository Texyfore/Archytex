use crate::{api::{fragment_render::FragmentRender, fragment_collector::FragmentCollector}, textures::texture_repo::TextureRepository};

use super::array_collector::ArrayCollector;

pub struct RawCollector{
    
}

impl<T: FragmentRender> FragmentCollector<T> for RawCollector{
    type Output = Vec<f32>;

    fn collect(
        &self,
        fragment_render: T,
        texture_repo: &TextureRepository,
        width: usize,
        height: usize,
    ) -> Self::Output {
        let collector = ArrayCollector{};
        collector.collect(fragment_render, texture_repo, width, height)
        .into_iter()
        .flatten()
        .map(|a| a.inner)
        .flatten()
        .map(|a| a as f32)
        .collect()
    }
}