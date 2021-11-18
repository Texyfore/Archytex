use std::{collections::HashMap, rc::Rc};

use crate::render::{GraphicsWorld, Texture};

#[derive(Default)]
pub struct TextureBank {
    textures: HashMap<u64, Rc<Texture>>,
}

impl TextureBank {
    pub fn add<G: GraphicsWorld>(&mut self, gfx: &G, uuid: u64, data: &[u8]) {
        if let Ok(image) = image::load_from_memory(data) {
            self.textures.insert(uuid, gfx.create_texture(&image));
        }
    }

    pub fn get(&self, uuid: u64) -> Option<Rc<Texture>> {
        self.textures.get(&uuid).cloned()
    }

    pub fn len(&self) -> usize {
        self.textures.len()
    }
}
