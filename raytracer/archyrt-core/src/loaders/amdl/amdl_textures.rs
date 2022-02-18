use std::{path::Path, fs::File, collections::HashMap};
use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::textures::{texture_repo::png::PngTextureRepo, TextureID};

#[derive(Serialize, Deserialize)]
struct Texture{
    pub id: u32,
    pub url: String
}

pub fn load(directory: &str) -> Result<PngTextureRepo>{
    let assetsjson = Path::new(directory).join("assets.json");
    let assetsjson = File::open(assetsjson)?;
    let json: Vec<Texture> = serde_json::from_reader(assetsjson)?;
    let mut textures = HashMap::new();
    for tex in json{
        textures.insert(TextureID(tex.id), PngTextureRepo::load(directory, tex.url.as_str())?);
    }
    Ok(PngTextureRepo{
        base: directory.into(),
        textures,
    })
}