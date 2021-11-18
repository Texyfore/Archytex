use std::rc::Rc;

use crate::render::{GraphicsWorld, Texture};

pub struct TextureBank {
    textures: Vec<Rc<Texture>>,
}

impl TextureBank {
    pub fn new<G: GraphicsWorld>(gfx: &G) -> Self {
        // Temporary solution as wasm is incapable of sending HTTP requests. Workaround needed.
        // Possibly let the browser download the textures and hand them over to the viewport?
        let textures = vec![
            gfx.create_texture(&image::load_from_memory(include_bytes!("nodraw.png")).unwrap())
        ];
        Self { textures }
    }

    pub fn get(&self, id: usize) -> Rc<Texture> {
        self.textures[id].clone()
    }

    pub fn len(&self) -> usize {
        self.textures.len()
    }
}
