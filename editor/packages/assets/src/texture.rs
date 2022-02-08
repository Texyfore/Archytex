use std::fmt::Debug;

use image::GenericImageView;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextureID(pub u32);

impl Debug for TextureID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

pub struct Texture {
    width: u32,
    height: u32,
    rgba8: Vec<u8>,
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
