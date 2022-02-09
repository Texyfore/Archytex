use std::collections::HashMap;

use asset::{GizmoID, GizmoVertex, PropID, PropVertex, TextureID};
use gpu::{Buffer, BufferUsages, Gpu, Image, Sampler, Texture};

#[derive(Default)]
pub struct Resources {
    textures: HashMap<TextureID, gpu::Texture>,
    props: HashMap<PropID, PropModel>,
    gizmos: HashMap<GizmoID, GizmoMesh>,
}

impl Resources {
    pub fn add_texture(
        &mut self,
        gpu: &Gpu,
        sampler: &Sampler,
        id: TextureID,
        texture: asset::Texture,
    ) {
        self.textures.insert(
            id,
            gpu.create_texture(
                sampler,
                Image {
                    width: texture.width,
                    height: texture.height,
                    buf: &texture.rgba8,
                },
            ),
        );
    }

    pub fn add_prop(&mut self, gpu: &Gpu, id: PropID, prop: asset::Prop) {
        self.props.insert(
            id,
            PropModel {
                meshes: prop
                    .meshes
                    .into_iter()
                    .map(|mesh| PropMesh {
                        texture: mesh.texture,
                        vertices: gpu.create_buffer(&mesh.vertices, BufferUsages::VERTEX),
                        triangles: gpu.create_buffer(&mesh.triangles, BufferUsages::INDEX),
                    })
                    .collect(),
            },
        );
    }

    pub fn add_gizmo(&mut self, gpu: &Gpu, id: GizmoID, gizmo: asset::Gizmo) {
        self.gizmos.insert(
            id,
            GizmoMesh {
                vertices: gpu.create_buffer(&gizmo.vertices, BufferUsages::VERTEX),
                triangles: gpu.create_buffer(&gizmo.triangles, BufferUsages::INDEX),
            },
        );
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
