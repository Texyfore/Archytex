use cgmath::Vector3;
use stable_vec::StableVec;

use crate::{
    editor::camera::WorldCamera,
    input::InputMapper,
    math::Ray,
    render::{SolidBatch, TextureID},
};

#[derive(Default)]
pub struct SolidContainer {
    points: StableVec<Point>,
    faces: StableVec<Face>,
    solids: StableVec<Solid>,
    selected: Option<Selection>,
}

impl SolidContainer {
    pub fn add(&mut self, position: Vector3<f32>, extent: Vector3<f32>) {}

    pub fn delete_selected(&mut self) {}

    pub fn move_selected(&mut self, vec: Vector3<f32>) {}

    pub fn reset_selected(&mut self) {}

    pub fn select_point(&mut self, input: &InputMapper) {}

    pub fn select_face(&mut self, camera: &WorldCamera) {}

    pub fn select_solid(&mut self, camera: &WorldCamera) {}

    pub fn deselect(&mut self) {
        self.selected = None;
    }

    pub fn raycast(&mut self, ray: Ray) -> Option<Raycast> {
        todo!()
    }

    pub fn build(&self) -> Vec<(TextureID, SolidBatch)> {
        todo!()
    }
}

pub struct Raycast {
    point: Vector3<f32>,
    solid: Option<RaycastSolid>,
}

pub struct RaycastSolid {
    solid: usize,
    face: usize,
}

struct Point {
    position: Vector3<f32>,
    previous: Vector3<f32>,
}

struct Face {
    quad: [usize; 4],
}

struct Solid {
    faces: [usize; 6],
}

enum Selection {
    Points(Vec<usize>),
    Faces(Vec<usize>),
    Solids(Vec<usize>),
}
