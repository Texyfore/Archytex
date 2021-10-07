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

    pub fn perspective(aspect: f32, fov: f32, near: f32, far: f32) -> Self {
        let tan_half_fov = (fov * 0.5).tan();
        let neg_span = near - far;

        let c00 = 1.0 / (aspect * tan_half_fov);
        let c11 = 1.0 / tan_half_fov;
        let c22 = (-near - far) / neg_span;
        let c32 = (2.0 * far - near) / neg_span;

        Self {
            #[rustfmt::skip]
            components: [
                c00, 0.0, 0.0, 0.0,
                0.0, c11, 0.0, 0.0,
                0.0, 0.0, c22, c32,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}

impl AsRef<[f32; 16]> for Mat4 {
    fn as_ref(&self) -> &[f32; 16] {
        &self.components
    }
}
