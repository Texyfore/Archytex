use asset::{PropID, TextureID};
use cgmath::{vec3, Vector3};

use crate::graphics::{Canvas, Graphics, PropData, PropInstance, Share, SolidMesh};

#[derive(Clone, Copy)]
pub enum ElementKind {
    Solid,
    Face,
    Point,
    Prop,
}

pub struct Solid {
    points: [Point; 8],
    faces: [Face; 6],
    selected: bool,
    mesh: SolidMesh,
}

impl Solid {
    pub fn new(graphics: &Graphics, origin: Vector3<i32>, extent: Vector3<i32>) -> Self {
        Self {
            points: [
                vec3(0, 0, 0).into(),
                vec3(1, 0, 0).into(),
                vec3(1, 0, 1).into(),
                vec3(0, 0, 1).into(),
                vec3(0, 1, 0).into(),
                vec3(1, 1, 0).into(),
                vec3(1, 1, 1).into(),
                vec3(0, 1, 1).into(),
            ],
            faces: [
                (TextureID(0), [1, 2, 6, 5]),
                (TextureID(0), [0, 0, 0, 0]),
                (TextureID(0), [0, 0, 0, 0]),
                (TextureID(0), [0, 0, 0, 0]),
                (TextureID(0), [0, 0, 0, 0]),
                (TextureID(0), [0, 0, 0, 0]),
            ],
            selected: todo!(),
            mesh: todo!(),
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_solid(self.mesh.share());
    }
}

pub struct Point {
    position: Vector3<i32>,
    selected: bool,
}

impl From<Vector3<i32>> for Point {
    fn from(position: Vector3<i32>) -> Self {
        Self {
            position,
            selected: false,
        }
    }
}

pub struct Face {
    texture: TextureID,
    indices: [usize; 4],
    selected: bool,
}

impl From<(TextureID, [usize; 4])> for Face {
    fn from(tuple: (TextureID, [usize; 4])) -> Self {
        Self {
            texture: tuple.0,
            indices: tuple.1,
            selected: false,
        }
    }
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
