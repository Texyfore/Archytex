use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct Vert {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable)]
pub struct Tri {
    pub idx: [u16; 3],
}
