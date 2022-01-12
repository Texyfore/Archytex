pub mod color_provider;
pub mod samplers;
pub mod texture;
pub mod texture_repo;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct TextureID(pub u32);
