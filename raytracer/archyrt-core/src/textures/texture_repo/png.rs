use std::{collections::HashMap, path::Path};

use anyhow::Result;
use image::io::Reader as ImageReader;

use crate::{
    textures::{texture::Texture, TextureID},
    vector,
};

use super::TextureRepository;

pub fn load_into(repo: &mut TextureRepository, base: &str, textures: &[(TextureID, &str)]) -> Result<()> {
    for (id, name) in textures {
        let texture = load(base, *name)?;
        repo.insert(*id, texture);
    }
    Ok(())
}
pub fn load(base: &str, name: &str) -> Result<Texture> {
    let path = Path::new(base).join(name);
    let image = ImageReader::open(path)?.decode()?;
    let image = image.into_rgb8();
    let pixels: Vec<_> = image
        .pixels()
        .map(|a| {
            vector![
                a.0[0] as f64 / 255.0,
                a.0[1] as f64 / 255.0,
                a.0[2] as f64 / 255.0
            ].from_srgb()
        })
        .collect();
    Ok(Texture {
        data: pixels,
        width: image.width(),
        height: image.height(),
    })
}