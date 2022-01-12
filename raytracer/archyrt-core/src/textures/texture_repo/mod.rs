use super::{TextureID, texture::Texture};

pub mod png;


pub trait TextureRepository{
    fn get(&self, id: TextureID) -> Option<Texture>;
}