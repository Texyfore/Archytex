pub mod linear;
pub mod nearest;

use crate::utilities::math::{Vec2, Vec3};

use super::texture::Texture;

pub trait TextureSampler {
    fn sample(&self, texture: &Texture, uv: Vec2) -> Vec3;
    fn sample_or_default(&self, texture: Option<&Texture>, uv: Vec2) -> Vec3 {
        texture.map(|texture| self.sample(texture, uv)).unwrap_or_default()
    }
}
