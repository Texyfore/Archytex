use crate::{input::Input, render::GraphicsWorld};

use super::{texture::TextureBank, EditMode};

#[derive(Default)]
pub struct BrushBank;

impl BrushBank {
    pub fn process<I, G>(&mut self, input: &I, gfx: &G, textures: &TextureBank, mode: &EditMode)
    where
        I: Input,
        G: GraphicsWorld,
    {
    }
}
