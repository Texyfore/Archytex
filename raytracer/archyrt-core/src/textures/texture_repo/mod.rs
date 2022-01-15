use super::{texture::Texture, TextureID};

pub mod png;

pub trait TextureRepository {
    fn get(&self, id: TextureID) -> Option<&Texture>;
}

impl<T: TextureRepository> TextureRepository for &T{
    fn get(&self, id: TextureID) -> Option<&Texture> {
        (*self).get(id)
    }
}

pub struct DummyTextureRepository();

impl TextureRepository for DummyTextureRepository {
    fn get(&self, _id: TextureID) -> Option<&Texture> {
        None
    }
}
