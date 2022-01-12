pub mod nearest;

use crate::utilities::math::{Vec2, Vec3};

use super::{texture_repo::TextureRepository, TextureID};

pub trait TextureSampler {
    fn sample<R: TextureRepository>(&self, repo: &R, texture: TextureID, uv: Vec2) -> Vec3;
}
