use tk3d::math::{
    perspective, vec2, vec3, Deg, Matrix4, SquareMatrix, Transform, Vector2, Vector3,
};

use crate::input::Input;

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector2<f32>,
    projection: Matrix4<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: vec3(20.0, 20.0, 20.0),
            rotation: vec2(-45.0, 45.0),
            projection: Matrix4::identity(),
        }
    }
}

impl Camera {
    pub fn process(&mut self, input: &Input) {}

    pub fn recreate_projection(&mut self, width: u32, height: u32) {
        self.projection = perspective(Deg(80.0), width as f32 / height as f32, 0.1, 256.0);
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.projection
            * (Matrix4::from_translation(self.position)
                * Matrix4::from_angle_y(Deg(self.rotation.y))
                * Matrix4::from_angle_x(Deg(self.rotation.x)))
            .inverse_transform()
            .unwrap()
    }
}
