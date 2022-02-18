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
}

impl AMDLTextureType {
    pub fn diffuse(id: u32) -> TextureID {
        TextureID::new(&Self::Diffuse(id))
    }
}

#[derive(Serialize, Deserialize)]
struct Texture {
    pub id: u32,
    pub url: String,
}

pub fn load_into(repo: &mut TextureRepository, directory: &str) -> Result<()> {
    let assetsjson = Path::new(directory).join("assets.json");
    let assetsjson = File::open(assetsjson)?;
    let json: Vec<Texture> = serde_json::from_reader(assetsjson)?;
    for tex in json {
        repo.insert(
            AMDLTextureType::diffuse(tex.id),
            png::load(directory, tex.url.as_str())?,
        );
    }
    Ok(())
}
