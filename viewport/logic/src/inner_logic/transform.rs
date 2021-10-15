use tools::math::{Deg, Matrix4, Quaternion, Rotation3, Vector3, Zero};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Vector3::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn calculate_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Into::<Matrix4<f32>>::into(self.rotation_quaternion())
            * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    pub fn rotation_quaternion(&self) -> Quaternion<f32> {
        Quaternion::from_angle_z(Deg(self.rotation.z))
            * Quaternion::from_angle_y(Deg(self.rotation.y))
            * Quaternion::from_angle_x(Deg(self.rotation.x))
    }

    pub fn right(&self) -> Vector3<f32> {
        self.rotation_quaternion() * Vector3::unit_x()
    }

    pub fn up(&self) -> Vector3<f32> {
        self.rotation_quaternion() * Vector3::unit_y()
    }

    pub fn forward(&self) -> Vector3<f32> {
        self.rotation_quaternion() * -Vector3::unit_z()
    }

    pub fn rotate(&mut self, rot: Vector3<f32>) {
        self.rotation += rot;
    }
}