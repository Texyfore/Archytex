use super::{texture::Texture, TextureID};

pub mod png;

pub trait TextureRepository {
    fn get(&self, id: TextureID) -> Option<&Texture>;
}

pub struct DummyTextureRepository();

impl TextureRepository for DummyTextureRepository {
    fn get(&self, _id: TextureID) -> Option<&Texture> {
        None
    }
}
