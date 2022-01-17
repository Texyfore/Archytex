use cgmath::Vector3;

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<[u16; 3]>,
}

#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: Vector3<f32>,
}

#[cfg(feature = "bytemuck")]
mod bytemuck_impl {
    use super::Vertex;
    use bytemuck::{Pod, Zeroable};

    unsafe impl Zeroable for Vertex {}
    unsafe impl Pod for Vertex {}
}
