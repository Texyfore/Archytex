use crate::utilities::math::Vec3;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Vec3>,
}

impl Texture {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: (0..width * height).map(|_| Vec3::default()).collect(),
        }
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn get(&self, index: usize) -> Option<Vec3> {
        if index >= self.data.len() {
            None
        } else {
            Some(self.data[index])
        }
    }
}
