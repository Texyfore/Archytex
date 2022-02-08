mod canvas;
mod renderer;

use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, SquareMatrix};

pub mod line;
pub mod solid;

pub use canvas::Canvas;
pub use renderer::Renderer;

pub trait Share {
    fn share(&self) -> Self;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Camera {
    pub world_to_clip: Matrix4<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            world_to_clip: Matrix4::identity(),
        }
    }
}

unsafe impl Zeroable for Camera {}
unsafe impl Pod for Camera {}

/*

* Colored lines
- Textured transformless solids with vertex tint
- Textured, transformed, multi-meshed props with mesh tint
- Unlit, colored gizmos that are instanced

*/
