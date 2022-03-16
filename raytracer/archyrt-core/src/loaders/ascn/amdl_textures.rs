use anyhow::{Result, anyhow};
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
struct Repo {
    pub textures: Vec<Texture>
}

#[derive(Serialize, Deserialize)]
struct Texture{
    pub id: u32,
    pub name: String,
    pub emissive: Option<String>
}

pub fn load_into(repo: &mut TextureRepository, directory: &str) -> Result<()> {
    let texturesjson = Path::new(directory).join("repo.json");
    let texturesjson = File::open(texturesjson)?;
    let json: Repo = serde_json::from_reader(texturesjson)?;
    for tex in json.textures {
        let textures_directory = Path::new(directory).join("textures");
        let textures_directory = textures_directory.to_str().ok_or(anyhow!("Unable to decode path string"))?;
        if let Some(emissive) = tex.emissive{
            repo.insert(
                AMDLTextureType::emissive(tex.id),
                png::load(textures_directory, &emissive)?,
            );
        }
        repo.insert(
            AMDLTextureType::diffuse(tex.id),
            png::load(textures_directory, &tex.name)?,
        );
    }
    Ok(())
}
