use tools::math::{Mat4, One, Quat, Vec3, Zero};

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: Vec3::zero(),
            rotation: Quat::one(),
            scale: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn calculate_matrix(&self) -> Mat4 {
        Mat4::from_translation(self.position)
            * Into::<Mat4>::into(self.rotation)
            * Mat4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
    }

    pub fn forwad(&self) -> Vec3 {
        self.rotation * -Vec3::unit_z()
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::unit_x()
    }
}
