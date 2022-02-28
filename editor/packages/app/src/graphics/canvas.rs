use std::rc::Rc;

use asset::{GizmoID, PropID, TextureID};
use gpu::{Buffer, Uniform};

use super::{
    structures::{
        CameraMatrices, GizmoInstance, GroundVertex, LineVertex, SolidVertex, TransformTint,
    },
    Share,
};

#[derive(Default)]
pub struct Canvas {
    pub(super) camera_matrices: CameraMatrices,
    pub(super) line_meshes: Vec<LineMesh>,
    pub(super) solid_meshes: Vec<SolidMesh>,
    pub(super) ground_meshes: Vec<GroundMesh>,
    pub(super) prop_instances: Vec<PropInstance>,
    pub(super) gizmo_groups: Vec<GizmoGroup>,
}

impl Canvas {
    pub fn set_camera_matrices(&mut self, matrices: CameraMatrices) {
        self.camera_matrices = matrices;
    }

    pub fn draw_lines(&mut self, line_mesh: LineMesh) {
        self.line_meshes.push(line_mesh);
    }

    pub fn draw_solid(&mut self, solid_mesh: SolidMesh) {
        self.solid_meshes.push(solid_mesh);
    }

    pub fn draw_ground(&mut self, ground_mesh: GroundMesh) {
        self.ground_meshes.push(ground_mesh);
    }

    pub fn draw_prop(&mut self, instance: PropInstance) {
        self.prop_instances.push(instance);
    }

    pub fn draw_gizmos(&mut self, group: GizmoGroup) {
        self.gizmo_groups.push(group);
    }
}

pub struct LineMesh {
    pub(super) vertices: Rc<Buffer<LineVertex>>,
}

impl Share for LineMesh {
    fn share(&self) -> Self {
        Self {
            vertices: self.vertices.clone(),
        }
    }
}

pub struct SolidMesh {
    pub textures: [TextureID; 6],
    pub(super) vertices: Rc<Buffer<SolidVertex>>,
    pub(super) triangles: Rc<Buffer<[u16; 3]>>,
}

impl Share for SolidMesh {
    fn share(&self) -> Self {
        Self {
            textures: self.textures,
            vertices: self.vertices.clone(),
            triangles: self.triangles.clone(),
        }
    }
}

pub struct GroundMesh {
    pub(super) texture: TextureID,
    pub(super) vertices: Rc<Buffer<GroundVertex>>,
    pub(super) triangles: Rc<Buffer<[u16; 3]>>,
}

impl Share for GroundMesh {
    fn share(&self) -> Self {
        Self {
            texture: self.texture,
            vertices: self.vertices.clone(),
            triangles: self.triangles.clone(),
        }
    }
}

pub struct PropInstance {
    pub prop: PropID,
    pub data: PropData,
}

pub struct PropData {
    pub(super) uniform: Rc<Uniform<TransformTint>>,
}

impl Share for PropData {
    fn share(&self) -> Self {
        Self {
            uniform: self.uniform.clone(),
        }
    }
}

pub struct GizmoGroup {
    pub gizmo: GizmoID,
    pub instances: GizmoInstances,
}

pub struct GizmoInstances {
    pub(super) buffer: Rc<Buffer<GizmoInstance>>,
    pub(super) len: u32,
}

impl Share for GizmoInstances {
    fn share(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            len: self.len,
        }
    }
}
