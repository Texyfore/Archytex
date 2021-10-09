use std::ops::Mul;

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

    pub fn rotation(euler_angles: Vec3) -> Self {
        let a = euler_angles.x;
        let b = euler_angles.y;
        let y = euler_angles.z;

        let c00 = a.cos() * b.cos();
        let c10 = a.cos() * b.sin() * y.sin() - a.sin() * y.cos();
        let c20 = a.cos() * b.sin() * y.cos() + a.sin() * y.sin();

        let c01 = a.sin() * b.cos();
        let c11 = a.sin() * b.sin() * y.sin() + a.cos() * y.cos();
        let c21 = a.sin() * b.sin() * y.cos() - a.cos() * y.sin();

        let c02 = -(b.sin());
        let c12 = b.cos() * y.sin();
        let c22 = b.cos() * y.cos();

        Self {
            #[rustfmt::skip]
            components: [
                c00, c10, c20, 0.0,
                c01, c11, c21, 0.0,
                c02, c12, c22, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn perspective(aspect: f32, fov: f32, near: f32, far: f32) -> Self {
        let tan_half_fov = (fov * 0.5).tan();
        let neg_span = near - far;

        let c00 = 1.0 / (aspect * tan_half_fov);
        let c11 = 1.0 / tan_half_fov;
        let c22 = (far - near) / neg_span;
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

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let a = &self;
        let b = &rhs;

        Self::Output {
            #[rustfmt::skip]
            components: [
                dot(a, b, 0, 0), dot(a, b, 0, 1), dot(a, b, 0, 2), dot(a, b, 0,3),
                dot(a, b, 1, 0), dot(a, b, 1, 1), dot(a, b, 1, 2), dot(a, b, 1,3),
                dot(a, b, 2, 0), dot(a, b, 2, 1), dot(a, b, 2, 2), dot(a, b, 2,3),
                dot(a, b, 3, 0), dot(a, b, 3, 1), dot(a, b, 3, 2), dot(a, b, 3,3),
            ],
        }
    }
}

impl AsRef<[f32; 16]> for Mat4 {
    fn as_ref(&self) -> &[f32; 16] {
        &self.components
    }
}

fn dot(a: &Mat4, b: &Mat4, r: usize, c: usize) -> f32 {
    a.components[r * 4] * b.components[c]
        + a.components[r * 4 + 1] * b.components[c + 4]
        + a.components[r * 4 + 2] * b.components[c + 4 * 2]
        + a.components[r * 4 + 3] * b.components[c + 4 * 3]
}
