use crate::{
    textures::{texture_repo::TextureRepository, TextureID},
    utilities::math::{Vec2, Vec3},
};

use super::TextureSampler;

pub struct NearestSampler {}

impl TextureSampler for NearestSampler {
    fn sample<R: TextureRepository>(&self, repo: &R, texture: TextureID, uv: Vec2) -> Vec3 {
        if let Some(texture) = repo.get(texture) {
            let w = texture.width() as usize;
            let h = texture.height() as usize;
            let x = uv.x() % 1.0;
            let y = uv.y() % 1.0;
            let x = if x < 0.0 { 1.0 + x } else { x };
            let y = if y < 0.0 { 1.0 + y } else { y };
            let x = ((x * (w as f64)) as usize) % w;
            let y = ((y * (h as f64)) as usize) % h;
            let index = y * w + x;
            texture.get(index).or(Some(Vec3::from_single(0.5))).unwrap()
        } else {
            return Vec3::from_single(1.0);
        }
    }
}
