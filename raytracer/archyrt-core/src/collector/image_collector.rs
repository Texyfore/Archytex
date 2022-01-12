use image::{Rgb, RgbImage};

use crate::{
    api::{fragment_collector::FragmentCollector, fragment_render::FragmentRender},
    textures::texture_repo::TextureRepository,
};

use super::array_collector::ArrayCollector;

pub struct ImageCollector {}

impl<T: FragmentRender> FragmentCollector<T> for ImageCollector {
    type Output = Option<RgbImage>;

    fn collect<R: TextureRepository>(
        &self,
        fragment_render: T,
        texture_repo: R,
        width: usize,
        height: usize,
    ) -> Self::Output {
        let array_collector = ArrayCollector {};
        let arr = array_collector.collect(fragment_render, texture_repo, width, height);
        let mut image = RgbImage::new(width as u32, height as u32);
        for (x, y, color) in image.enumerate_pixels_mut() {
            let fragment = arr.get(y as usize)?.get(x as usize)?;
            let r = fragment.x() * 255.0;
            let g = fragment.y() * 255.0;
            let b = fragment.z() * 255.0;
            let r = r.clamp(0.0, 255.0);
            let g = g.clamp(0.0, 255.0);
            let b = b.clamp(0.0, 255.0);
            let r = r as u8;
            let g = g as u8;
            let b = b as u8;
            *color = Rgb([r, g, b]);
        }
        Some(image)
    }
}
