pub mod nearest;
pub mod linear;

use crate::utilities::math::{Vec2, Vec3};

use super::texture::Texture;

pub trait TextureSampler {
    fn sample(&self, texture: &Texture, uv: Vec2) -> Vec3;
    fn sample_or_default(&self, texture: Option<&Texture>, uv: Vec2) -> Vec3 {
        texture
            .and_then(|texture| Some(self.sample(texture, uv)))
            .unwrap_or(Vec3::default())
    }
}
