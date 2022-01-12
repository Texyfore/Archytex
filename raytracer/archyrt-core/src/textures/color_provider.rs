use crate::utilities::math::Vec3;

use super::texture_repo::TextureRepository;

pub trait ColorProvider {
    fn get_color<R: TextureRepository>(&self, repo: &R) -> Vec3;
}

#[derive(Default)]
pub struct SolidColor(pub Vec3);

impl ColorProvider for SolidColor {
    fn get_color<R: TextureRepository>(&self, _: &R) -> Vec3 {
        self.0
    }
}
