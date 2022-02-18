use std::path::Path;

use crate::{
    textures::{texture::Texture, TextureID},
    utilities::math::Vec3,
};

use super::TextureRepository;
use anyhow::Result;
use exr::prelude::*;

pub fn load_into(
    repo: &mut TextureRepository,
    base: &str,
    textures: &[(TextureID, &str)],
) -> Result<()> {
    for (id, name) in textures {
        let texture = load(base, *name)?;
        repo.insert(*id, texture);
    }
    Ok(())
}
pub fn load(base: &str, name: &str) -> Result<Texture> {
    let path = Path::new(base).join(name);
    let image = read_first_rgba_layer_from_file(
        path,
        |size, _| Texture::new(size.x() as u32, size.y() as u32),
        |image, coords, (r, g, b, _): (f32, f32, f32, f32)| {
            let color = Vec3::new(r as f64, g as f64, b as f64);
            let index = coords.y() * (image.width as usize) + coords.x();
            image.data[index] = color;
        },
    )?;
    let texture = image.layer_data.channel_data.pixels;
    Ok(texture)
}
