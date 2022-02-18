use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub mod color_provider;
pub mod samplers;
pub mod texture;
pub mod texture_repo;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct TextureID(u64);

impl TextureID{
    pub fn new<T: Hash>(val: &T) -> Self{
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        Self(hasher.finish())
    }
}

impl Default for TextureID{
    fn default() -> Self {
        Self::new(&1u32)
    }
}