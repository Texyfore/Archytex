use std::rc::Rc;

use asset_id::TextureID;
use cgmath::Matrix4;

use crate::data::{GizmoInstances, GizmoMesh, Lines, Mesh, Transform};

#[derive(Default)]
pub struct Scene {
    pub(crate) camera_matrix: [[f32; 4]; 4],
    pub(crate) mesh_objects: Vec<MeshObject>,
    pub(crate) line_objects: Vec<LineObject>,
    pub(crate) gizmo_objects: Vec<GizmoObject>,
}

impl Scene {
    pub fn set_camera_matrix(&mut self, matrix: Matrix4<f32>) {
        self.camera_matrix = matrix.into();
    }

    pub fn push_mesh_object(&mut self, mesh_object: MeshObject) {
        self.mesh_objects.push(mesh_object);
    }

    pub fn push_line_object(&mut self, line_object: LineObject) {
        self.line_objects.push(line_object);
    }

    pub fn push_gizmo_object(&mut self, gizmo_object: GizmoObject) {
        self.gizmo_objects.push(gizmo_object);
    }
}

pub struct MeshObject {
    pub texture_id: TextureID,
    pub transform: Rc<Transform>,
    pub mesh: Rc<Mesh>,
}

pub struct LineObject {
    pub transform: Rc<Transform>,
    pub lines: Rc<Lines>,
}

pub struct GizmoObject {
    pub mesh: Rc<GizmoMesh>,
    pub instances: Rc<GizmoInstances>,
}
