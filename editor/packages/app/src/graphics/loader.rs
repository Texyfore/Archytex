use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

use asset::{BoundingBox, GizmoID, PropID, TextureID};
use gpu::{BufferUsages, Gpu, Image, Sampler, Texture};

use crate::{Resource, ResourceKind};

use super::renderer::{GizmoMesh, PropMesh, PropModel};

pub struct ResourceLoader {
    ctx: Sender<Command>,
    rrx: Receiver<LoadedResource>,
    handle: Option<JoinHandle<()>>,
}

impl ResourceLoader {
    pub fn new(gpu: Arc<Gpu>, sampler: Sampler) -> Self {
        let (rtx, rrx) = channel();
        let (ctx, crx) = channel();

        let handle = thread::spawn(move || {
            while let Ok(cmd) = crx.recv() {
                match cmd {
                    Command::Load(resource) => match resource.kind {
                        ResourceKind::Texture => {
                            let id = TextureID(resource.id);
                            let texture = load_texture(&gpu, &sampler, &resource.buf);
                            rtx.send(LoadedResource::Texture { id, texture }).unwrap();
                        }
                        ResourceKind::Prop => {
                            let id = PropID(resource.id);
                            let (bounds, model) = load_prop(&gpu, &resource.buf);
                            rtx.send(LoadedResource::Prop { id, bounds, model })
                                .unwrap();
                        }
                        ResourceKind::Gizmo => {
                            let id = GizmoID(resource.id);
                            let mesh = load_gizmo(&gpu, &resource.buf);
                            rtx.send(LoadedResource::Gizmo { id, mesh }).unwrap();
                        }
                    },
                    Command::Exit => break,
                }
            }
        });

        Self {
            ctx,
            rrx,
            handle: Some(handle),
        }
    }

    pub fn push_job(&self, resource: Resource) {
        self.ctx.send(Command::Load(resource)).unwrap();
    }

    pub fn poll(&self) -> Option<LoadedResource> {
        self.rrx.try_recv().ok()
    }
}

impl Drop for ResourceLoader {
    fn drop(&mut self) {
        self.ctx.send(Command::Exit).unwrap();
        self.handle.take().unwrap().join().unwrap();
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

enum Command {
    Load(Resource),
    Exit,
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
