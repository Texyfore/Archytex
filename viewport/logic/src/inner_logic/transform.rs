use tools::math::{Matrix4, One, Quaternion, Vector3, Zero};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Quaternion::one(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn calculate_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Into::<Matrix4<f32>>::into(self.rotation)
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.rotation * -Vector3::unit_z()
    }

    pub fn right(&self) -> Vector3<f32> {
        self.rotation * Vector3::unit_x()
    }
}
