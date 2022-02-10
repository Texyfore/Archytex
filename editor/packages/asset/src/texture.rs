use std::fmt::Debug;

use image::GenericImageView;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TextureID(pub u32);

impl Debug for TextureID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
}

impl Texture {
    pub fn new(buf: &[u8]) -> Self {
        let image = image::load_from_memory(buf).unwrap();
        let (width, height) = image.dimensions();
        let rgba8 = image.into_rgba8().to_vec();

        Self {
            width,
            height,
            rgba8,
        }
    }
}
