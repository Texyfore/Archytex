use cgmath::{Matrix4, SquareMatrix};
use gpu::data::Uniform;

use crate::Renderer;

pub struct Transform {
    pub(crate) uniform: Uniform<[[f32; 4]; 4]>,
}

pub struct Tint {
    pub(crate) uniform: Uniform<[f32; 4]>,
}

impl Renderer {
    pub fn create_transform(&self) -> Transform {
        let uniform = self.gpu.create_uniform(&self.uniform_layout);
        self.gpu.set_uniform(&uniform, &Matrix4::identity().into());
        Transform { uniform }
    }

    pub fn set_transform(&self, transform: &Transform, matrix: Matrix4<f32>) {
        self.gpu.set_uniform(&transform.uniform, &matrix.into());
    }

    pub fn create_tint(&self) -> Tint {
        let uniform = self.gpu.create_uniform(&self.uniform_layout);
        self.gpu.set_uniform(&uniform, &[0.0; 4]);
        Tint { uniform }
    }
}
