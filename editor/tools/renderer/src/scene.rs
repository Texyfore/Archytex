use std::rc::Rc;

use asset_id::TextureID;
use cgmath::{Matrix4, Transform as CgMathTransform};

use crate::data::{gizmo, line, solid, transform::Transform};

#[derive(Default)]
pub struct Scene {
    pub(crate) camera_world: [[f32; 4]; 4],
    pub(crate) camera_clip: [[f32; 4]; 4],
    pub(crate) solid_objects: Vec<SolidObject>,
    pub(crate) line_objects: Vec<LineObject>,
    pub(crate) gizmo_objects: Vec<GizmoObject>,
}

impl Scene {
    pub fn set_camera_matrices(&mut self, world: Matrix4<f32>, projection: Matrix4<f32>) {
        self.camera_world = world.into();
        self.camera_clip = (projection * world.inverse_transform().unwrap()).into();
    }

    pub fn push_solid_object(&mut self, solid_object: SolidObject) {
        self.solid_objects.push(solid_object);
    }

    pub fn push_line_object(&mut self, line_object: LineObject) {
        self.line_objects.push(line_object);
    }

    pub fn push_gizmo_object(&mut self, gizmo_object: GizmoObject) {
        self.gizmo_objects.push(gizmo_object);
    }
}

#[derive(Clone)]
pub struct SolidObject {
    pub texture_id: TextureID,
    pub transform: Rc<Transform>,
    pub mesh: Rc<solid::Mesh>,
}

#[derive(Clone)]
pub struct LineObject {
    pub transform: Rc<Transform>,
    pub lines: Rc<line::Mesh>,
}

#[derive(Clone)]
pub struct GizmoObject {
    pub mesh: Rc<gizmo::Mesh>,
    pub instances: Rc<gizmo::Instances>,
}
