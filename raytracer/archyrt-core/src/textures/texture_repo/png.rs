use std::{collections::HashMap, path::Path};

use anyhow::Result;
use image::io::Reader as ImageReader;

use crate::{
    textures::{texture::Texture, TextureID},
    vector,
};

use super::TextureRepository;

pub struct PngTextureRepo {
    pub base: String,
    pub textures: HashMap<TextureID, Texture>,
}

//TODO: Load PngTextureRepo from file

impl PngTextureRepo {
    pub fn new(base: &str, textures: &[(TextureID, &str)]) -> Result<Self> {
        let mut t = HashMap::new();
        for (id, name) in textures {
            let texture = Self::generate_texture(base, *name)?;
            t.insert(*id, texture);
        }
        Ok(Self {
            base: base.into(),
            textures: t,
        })
    }
    pub fn generate_texture(base: &str, name: &str) -> Result<Texture> {
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
                ]
            })
            .collect();
        Ok(Texture {
            data: pixels,
            width: image.width(),
            height: image.height(),
        })
    }
}

impl TextureRepository for PngTextureRepo {
    fn get(&self, id: TextureID) -> Option<&Texture> {
        let texture = self.textures.get(&id)?;
        Some(texture)
    }
}
