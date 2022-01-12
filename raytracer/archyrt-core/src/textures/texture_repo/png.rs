use std::{collections::HashMap, path::Path};

use image::io::Reader as ImageReader;

use crate::{textures::{TextureID, texture::Texture}, vector};

use super::TextureRepository;

pub struct PngTextureRepo{
    pub base: String,
    pub textures: HashMap<TextureID, String>
}

//TODO: Load PngTextureRepo from file

impl TextureRepository for PngTextureRepo{
    fn get(&self, id: TextureID) -> Option<Texture> {
        let name = self.textures.get(&id)?;
        let path = Path::new(&self.base).join(name);
        let image = ImageReader::open(path).ok()?.decode().ok()?;
        let image = image.as_rgb8()?;
        let pixels: Vec<_> = image.pixels().map(|a|vector![a.0[0] as f64 / 255.0, a.0[1] as f64 / 255.0, a.0[2] as f64 / 255.0]).collect();
        Some(
            Texture{
                data: pixels,
                width: image.width(),
                height: image.height()
            }
        )
    }
}