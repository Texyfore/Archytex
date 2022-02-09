use std::rc::Rc;

use asset::{PropID, TextureID};
use gpu::{Buffer, Uniform};

use super::{
    structures::{CameraMatrices, LineVertex, SolidVertex, TransformTint},
    Share,
};

#[derive(Default)]
pub struct Canvas {
    pub(super) camera_matrices: CameraMatrices,
    pub(super) line_meshes: Vec<LineMesh>,
    pub(super) solid_meshes: Vec<SolidMesh>,
    pub(super) prop_instances: Vec<PropInstance>,
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

    pub fn draw_prop(&mut self, instance: PropInstance) {
        self.prop_instances.push(instance);
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
    pub(super) texture: TextureID,
    pub(super) vertices: Rc<Buffer<SolidVertex>>,
    pub(super) triangles: Rc<Buffer<[u16; 3]>>,
}

impl Share for SolidMesh {
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
