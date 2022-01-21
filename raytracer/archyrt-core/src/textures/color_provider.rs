use crate::{utilities::math::Vec3, renderers::path_tracer::Material};

use super::texture_repo::TextureRepository;

pub trait ColorProvider {
    fn get_color<R: TextureRepository>(&self, repo: &R) -> Vec3;
    fn get_material(&self) -> Material;
}

#[derive(Default, Clone)]
pub struct SolidColor(pub Vec3, pub Material);

impl ColorProvider for SolidColor {
    fn get_color<R: TextureRepository>(&self, _: &R) -> Vec3 {
        self.0
    }

    fn get_material(&self) -> Material {
        self.1
    }
    
}
