use std::collections::HashMap;

use asset::{GizmoID, GizmoVertex, PropID, PropVertex, TextureID};
use gpu::{Buffer, Texture};

#[derive(Default)]
pub struct Resources {
    textures: HashMap<TextureID, gpu::Texture>,
    props: HashMap<PropID, PropModel>,
    gizmos: HashMap<GizmoID, GizmoMesh>,
}

impl Resources {
    pub fn add_texture(&mut self, id: TextureID, texture: Texture) {
        self.textures.insert(id, texture);
    }

    pub fn add_prop(&mut self, id: PropID, model: PropModel) {
        self.props.insert(id, model);
    }

    pub fn add_gizmo(&mut self, id: GizmoID, mesh: GizmoMesh) {
        self.gizmos.insert(id, mesh);
    }

    pub fn texture(&self, id: TextureID) -> Option<&Texture> {
        self.textures.get(&id)
    }

    pub fn prop(&self, id: PropID) -> Option<&PropModel> {
        self.props.get(&id)
    }

    pub fn gizmo(&self, id: GizmoID) -> Option<&GizmoMesh> {
        self.gizmos.get(&id)
    }
}

pub struct PropModel {
    pub meshes: Vec<PropMesh>,
}

pub struct PropMesh {
    pub texture: TextureID,
    pub vertices: Buffer<PropVertex>,
    pub triangles: Buffer<[u16; 3]>,
}

pub struct GizmoMesh {
    pub vertices: Buffer<GizmoVertex>,
    pub triangles: Buffer<[u16; 3]>,
}
