use bytemuck::{Pod, Zeroable};
use gpu::data::Uniform;

use crate::Renderer;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Info {
    pub step: i32,
    pub snap_flag: i32,
}

pub struct InfoHolder {
    pub(crate) uniform: Uniform<Info>,
}

impl Renderer {
    pub fn create_grid_info_holder(&self, info: &Info) -> InfoHolder {
        let uniform = self.gpu.create_uniform(&self.uniform_layout);
        self.gpu.set_uniform(&uniform, info);
        InfoHolder { uniform }
    }
}
