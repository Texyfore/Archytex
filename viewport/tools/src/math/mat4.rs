use super::Vec3;

#[derive(Clone, Copy)]
pub struct Mat4 {
    components: [f32; 16],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            #[rustfmt::skip]
            components: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn translation(translation: Vec3) -> Self {
        let t = translation;
        Self {
            #[rustfmt::skip]
            components: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                t.x, t.y, t.z, 1.0,
            ],
        }
    }
}

impl AsRef<[f32; 16]> for Mat4 {
    fn as_ref(&self) -> &[f32; 16] {
        &self.components
    }
}
