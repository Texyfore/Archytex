use crate::{
    textures::{texture_repo::TextureRepository, TextureID, texture::Texture},
    utilities::math::{Vec2, Vec3}, vector,
};

use super::{TextureSampler, nearest::NearestSampler};

pub struct LinearSampler {}

impl TextureSampler for LinearSampler {
    fn sample(&self, texture: &Texture, uv: Vec2) -> Vec3 {
        let sampler = NearestSampler{};
        let w = texture.width() as usize;
        let h = texture.height() as usize;
        let x = uv.x() * (w as f64);
        let y = uv.y() * (h as f64);

        let x1 = x.floor();
        let x2 = x1+1.0;
        let y1 = y.floor();
        let y2 = y1+1.0;

        let xt = x - x1;
        let yt = y - y1;

        let x1 = x1 / (w as f64);
        let x2 = x2 / (w as f64);
        let y1 = y1 / (h as f64);
        let y2 = y2 / (h as f64);

        let c1 = sampler.sample(texture, vector![x1, y1]);
        let c2 = sampler.sample(texture, vector![x2, y1]);
        let c3 = sampler.sample(texture, vector![x1, y2]);
        let c4 = sampler.sample(texture, vector![x2, y2]);

        let c1 = c1 * (1.0-xt) + c2 * xt;
        let c2 = c3 * (1.0-xt) + c4 * xt;
        let c = c1 * (1.0-yt) + c2 * yt;
        c
    }
}
