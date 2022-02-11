use asset::{PropID, TextureID};
use cgmath::Vector3;

use crate::graphics::{Canvas, PropData, PropInstance, Share, SolidMesh};

#[derive(Clone, Copy)]
pub enum ElementKind {
    Solid,
    Face,
    Point,
    Prop,
}

pub struct Solid {
    faces: [Face; 6],
    points: [Point; 8],
    selected: bool,
    mesh: SolidMesh,
}

impl Solid {
    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_solid(self.mesh.share());
    }
}

pub struct Face {
    texture: TextureID,
    indices: [usize; 4],
    selected: bool,
}

pub struct Point {
    position: Vector3<i32>,
    selected: bool,
}

pub struct Prop {
    asset: PropID,
    position: Vector3<i32>,
    rotation: Vector3<i32>,
    data: PropData,
}

impl Prop {
    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_prop(PropInstance {
            prop: self.asset,
            data: self.data.share(),
        });
    }
}
