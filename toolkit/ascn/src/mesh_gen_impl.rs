use cgmath::Vector3;

use crate::{Face, Model, Point, Solid};

impl mesh_gen::Model<Solid, Face, Point> for Model {
    fn solids(&self) -> &[Solid] {
        &self.solids
    }
}

impl mesh_gen::Solid<Face, Point> for Solid {
    fn faces(&self) -> &[Face; 6] {
        &self.faces
    }

    fn points(&self) -> &[Point; 8] {
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

impl mesh_gen::Point for Point {
    fn position(&self) -> Vector3<f32> {
        self.position
    }
}
