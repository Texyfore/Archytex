use std::rc::Rc;

use asset::{BoundingBox, GizmoID, PropID, TextureID};
use gpu::{BufferUsages, Gpu, Image, Sampler, Texture};

use crate::{Resource, ResourceKind};

use super::renderer::{GizmoMesh, PropMesh, PropModel};

pub struct ResourceLoader {
    gpu: Rc<Gpu>,
    sampler: Sampler,
    jobs: Vec<Resource>,
}

impl ResourceLoader {
    pub fn new(gpu: Rc<Gpu>, sampler: Sampler) -> Self {
        Self {
            gpu,
            sampler,
            jobs: Vec::new(),
        }
    }

    pub fn push_job(&mut self, resource: Resource) {
        self.jobs.push(resource);
    }

    pub fn process(&mut self) -> Option<LoadedResource> {
        if let Some(resource) = self.jobs.pop() {
            match resource.kind {
                ResourceKind::Texture => {
                    let id = TextureID(resource.id);
                    let texture = load_texture(&self.gpu, &self.sampler, &resource.buf);
                    return Some(LoadedResource::Texture { id, texture });
                }
                ResourceKind::Prop => {
                    let id = PropID(resource.id);
                    let (bounds, model) = load_prop(&self.gpu, &resource.buf);
                    return Some(LoadedResource::Prop { id, bounds, model });
                }
                ResourceKind::Gizmo => {
                    let id = GizmoID(resource.id);
                    let mesh = load_gizmo(&self.gpu, &resource.buf);
                    return Some(LoadedResource::Gizmo { id, mesh });
                }
            }
        }

        None
    }
}

pub enum LoadedResource {
    Texture {
        id: TextureID,
        texture: Texture,
    },
    Prop {
        id: PropID,
        bounds: BoundingBox,
        model: PropModel,
    },
    Gizmo {
        id: GizmoID,
        mesh: GizmoMesh,
    },
}

fn load_texture(gpu: &Gpu, sampler: &Sampler, buf: &[u8]) -> Texture {
    let texture = asset::Texture::new(buf);
    gpu.create_texture(
        sampler,
        Image {
            width: texture.width,
            height: texture.height,
            buf: &texture.rgba8,
        },
    )
}

fn load_prop(gpu: &Gpu, buf: &[u8]) -> (BoundingBox, PropModel) {
    let prop = asset::Prop::decode(buf).unwrap();
    let model = PropModel {
        meshes: prop
            .meshes
            .into_iter()
            .map(|mesh| PropMesh {
                texture: mesh.texture,
                vertices: gpu.create_buffer(&mesh.vertices, BufferUsages::VERTEX),
                triangles: gpu.create_buffer(&mesh.triangles, BufferUsages::INDEX),
            })
            .collect(),
    };

    (prop.bounds, model)
}

fn load_gizmo(gpu: &Gpu, buf: &[u8]) -> GizmoMesh {
    let gizmo = asset::Gizmo::decode(buf).unwrap();

    GizmoMesh {
        vertices: gpu.create_buffer(&gizmo.vertices, BufferUsages::VERTEX),
        triangles: gpu.create_buffer(&gizmo.triangles, BufferUsages::INDEX),
    }
}
