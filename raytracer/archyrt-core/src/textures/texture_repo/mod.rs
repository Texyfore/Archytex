use std::collections::HashMap;

use super::{texture::Texture, TextureID};

pub mod png;

pub struct TextureRepository {
    pub textures: HashMap<TextureID, Texture>,
}

impl TextureRepository{
    pub fn new() -> Self {
        let t = HashMap::new();
        Self {
            textures: t,
        }
    }
    pub fn get(&self, id: TextureID) -> Option<&Texture> {
        let texture = self.textures.get(&id)?;
        Some(texture)
    }
    pub fn insert(&mut self, id: TextureID, texture: Texture){
        self.textures.insert(id, texture);
    }
}
