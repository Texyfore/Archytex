use std::rc::Rc;

use asset_id::{GizmoID, PropID, TextureID};
use cgmath::{Matrix4, Transform as CgMathTransform};

use crate::data::{gizmo, grid, line, solid, uniform::Transform};

#[derive(Default)]
pub struct Scene {
    pub(crate) camera_world: [[f32; 4]; 4],
    pub(crate) camera_clip: [[f32; 4]; 4],
    pub(crate) solid_objects: Vec<SolidObject>,
    pub(crate) prop_objects: Vec<PropObject>,
    pub(crate) line_objects: Vec<LineObject>,
    pub(crate) gizmo_objects: Vec<GizmoObject>,
    pub(crate) grid_objects: Vec<GridObject>,
}

impl Scene {
    pub fn set_camera_matrices(&mut self, world: Matrix4<f32>, projection: Matrix4<f32>) {
        self.camera_world = world.into();
        self.camera_clip = (projection * world.inverse_transform().unwrap()).into();
    }

    pub fn push_solid_object(&mut self, solid_object: SolidObject) {
        self.solid_objects.push(solid_object);
    }

    pub fn push_prop_object(&mut self, prop_object: PropObject) {
        self.prop_objects.push(prop_object);
    }

    pub fn push_line_object(&mut self, line_object: LineObject) {
        self.line_objects.push(line_object);
    }

    pub fn push_gizmo_object(&mut self, gizmo_object: GizmoObject) {
        self.gizmo_objects.push(gizmo_object);
    }

    pub fn push_grid_object(&mut self, grid_object: GridObject) {
        self.grid_objects.push(grid_object);
    }
}

#[derive(Clone)]
pub struct SolidObject {
    pub texture: TextureID,
    pub transform: Rc<Transform>,
    pub mesh: Rc<solid::Mesh>,
}

#[derive(Clone)]
pub struct PropObject {
    pub prop: PropID,
    pub transform: Rc<Transform>,
    pub tint: [f32; 4],
}

#[derive(Clone)]
pub struct LineObject {
    pub transform: Rc<Transform>,
    pub lines: Rc<line::Mesh>,
}

#[derive(Clone)]
pub struct GizmoObject {
    pub id: GizmoID,
    pub instances: Rc<gizmo::Instances>,
}

#[derive(Clone)]
pub struct GridObject {
    pub lines: Rc<line::Mesh>,
    pub info: Rc<grid::InfoHolder>,
}
