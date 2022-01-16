use std::rc::Rc;

use tk3d::{math::Matrix4, TextureID};

use crate::data::{GizmoInstances, GizmoMesh, Lines, Mesh, Transform};

#[derive(Default)]
pub struct Scene {
    pub(crate) camera_matrix: [[f32; 4]; 4],
    pub(crate) mesh_objects: Vec<Rc<MeshObject>>,
    pub(crate) line_objects: Vec<Rc<LineObject>>,
    pub(crate) gizmo_objects: Vec<Rc<GizmoObject>>,
}

impl Scene {
    pub fn set_camera_matrix(&mut self, matrix: Matrix4<f32>) {
        self.camera_matrix = matrix.into();
    }

    pub fn push_mesh_object(&mut self, mesh_object: Rc<MeshObject>) {
        self.mesh_objects.push(mesh_object);
    }

    pub fn push_line_object(&mut self, line_object: Rc<LineObject>) {
        self.line_objects.push(line_object);
    }

    pub fn push_gizmos(&mut self, gizmo_object: Rc<GizmoObject>) {
        self.gizmo_objects.push(gizmo_object);
    }
}

pub struct MeshObject {
    pub texture_id: TextureID,
    pub transform: Transform,
    pub mesh: Mesh,
}

pub struct LineObject {
    pub transform: Transform,
    pub lines: Lines,
}

pub struct GizmoObject {
    pub mesh: GizmoMesh,
    pub instances: GizmoInstances,
}
