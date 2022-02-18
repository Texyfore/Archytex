use crate::{renderers::path_tracer::Material, utilities::math::Vec3};

use super::texture_repo::TextureRepository;

pub trait ColorProvider {
    fn get_color(&self, repo: &TextureRepository) -> Vec3;
    fn get_material(&self) -> Material;
}

#[derive(Default, Clone)]
pub struct SolidColor(pub Vec3, pub Material);

impl ColorProvider for SolidColor {
    fn get_color(&self, _: &TextureRepository) -> Vec3 {
        self.0
    }

    fn get_material(&self) -> Material {
        self.1
    }
}
