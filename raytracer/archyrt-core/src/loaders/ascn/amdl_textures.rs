use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

use crate::textures::{
    texture_repo::{png, TextureRepository},
    TextureID,
};

#[derive(Hash)]
pub enum AMDLTextureType {
    Diffuse(u32),
    Emissive(u32),
}

impl AMDLTextureType {
    pub fn diffuse(id: u32) -> TextureID {
        TextureID::new(&Self::Diffuse(id))
    }
    pub fn emissive(id: u32) -> TextureID {
        TextureID::new(&Self::Emissive(id))
    }
}

#[derive(Serialize, Deserialize)]
struct Texture {
    pub id: u32,
    pub url: String,
    pub emissive: Option<String>
}

pub fn load_into(repo: &mut TextureRepository, directory: &str) -> Result<()> {
    let texturesjson = Path::new(directory).join("textures.json");
    let texturesjson = File::open(texturesjson)?;
    let json: Vec<Texture> = serde_json::from_reader(texturesjson)?;
    for tex in json {
        if let Some(emissive) = tex.emissive{
            repo.insert(
                AMDLTextureType::emissive(tex.id),
                png::load(directory, emissive.as_str())?,
            );
        }
        repo.insert(
            AMDLTextureType::diffuse(tex.id),
            png::load(directory, tex.url.as_str())?,
        );
    }
    Ok(())
}