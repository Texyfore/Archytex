use cgmath::Vector3;

use crate::{Face, Model, Solid};

impl mesh_gen::Model<Face, Solid> for Model {
    fn solids(&self) -> &[Solid] {
        &self.solids
    }
}

impl mesh_gen::Solid<Face> for Solid {
    fn faces(&self) -> &[Face] {
        &self.faces
    }

    fn points(&self) -> &[Vector3<f32>; 8] {
        &self.points
    }
}

impl mesh_gen::Face for Face {
    fn texture_id(&self) -> asset_id::TextureID {
        self.texture_id
    }

    fn points(&self) -> &[usize; 4] {
        &self.points
    }
}
