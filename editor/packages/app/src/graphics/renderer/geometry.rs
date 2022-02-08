use asset::TextureID;
use bytemuck::{Pod, Zeroable};
use cgmath::{Vector2, Vector3};
use gpu::{Buffer, BufferUsages, Gpu};

pub(super) struct PropModel {
    pub meshes: Vec<PropMesh>,
}

impl PropModel {
    pub fn new(gpu: &Gpu, prop: asset::Prop) -> Self {
        Self {
            meshes: prop
                .meshes
                .into_iter()
                .map(|mesh| PropMesh::new(gpu, mesh))
                .collect(),
        }
    }
}

pub(super) struct PropMesh {
    pub texture: TextureID,
    pub vertices: Buffer<PropVertex>,
    pub triangles: Buffer<[u16; 3]>,
}

impl PropMesh {
    pub fn new(gpu: &Gpu, mesh: asset::prop::Mesh) -> Self {
        let vertices = mesh
            .vertices
            .into_iter()
            .map(|vertex| vertex.into())
            .collect::<Vec<_>>();

        Self {
            texture: mesh.texture,
            vertices: gpu.create_buffer(&vertices, BufferUsages::VERTEX),
            triangles: gpu.create_buffer(&mesh.triangles, BufferUsages::INDEX),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct PropVertex {
    position: Vector3<f32>,
    normal: Vector3<f32>,
    texcoord: Vector2<f32>,
}

impl From<asset::prop::Vertex> for PropVertex {
    fn from(vertex: asset::prop::Vertex) -> Self {
        Self {
            position: vertex.position,
            normal: vertex.normal,
            texcoord: vertex.texcoord,
        }
    }
}

unsafe impl Zeroable for PropVertex {}
unsafe impl Pod for PropVertex {}
