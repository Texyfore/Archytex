pub mod color_provider;
pub mod texture;
pub mod texture_repo;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TextureID(pub u32);