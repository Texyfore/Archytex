use bytemuck::{Pod, Zeroable};

pub type Position = [f32; 3];
pub type Normal = [f32; 3];
pub type TexCoord = [f32; 2];
pub type Color = [f32; 4];
pub type Triangle = [u16; 3];

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct LineVertex {
    pub position: Position,
    pub color: Color,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct BrushVertex {
    pub position: Position,
    pub normal: Normal,
    pub texcoord: TexCoord,
}
