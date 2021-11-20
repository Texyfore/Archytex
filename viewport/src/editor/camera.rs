use cgmath::{
    num_traits::clamp, perspective, Deg, Matrix3, Matrix4, SquareMatrix, Vector2, Vector3, Zero,
};

use crate::input::InputMapper;

use super::ActionBinding::*;

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector2<f32>,
    speed: f32,
    sensitivity: f32,
    fov: f32,
    near: f32,
    far: f32,
    projection: Matrix4<f32>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Vector2::zero(),
            speed: 0.1,
            sensitivity: 0.1,
            fov: 80.0,
            near: 0.1,
            far: 100.0,
            projection: Matrix4::identity(),
        }
    }
}

impl Camera {
    pub fn process(&mut self, input: &InputMapper, matrix: &mut Matrix4<f32>) {
        if !input.is_active(EnableCameraMovement) {
            return;
        }

        if input.is_active(Forward) {
            self.position += self.forward() * self.speed;
        }

        if input.is_active(Backward) {
            self.position -= self.forward() * self.speed;
        }

        if input.is_active(Left) {
            self.position -= self.right() * self.speed;
        }

        if input.is_active(Right) {
            self.position += self.right() * self.speed;
        }

        if input.is_active(Up) {
            self.position += Vector3::unit_y() * self.speed;
        }

        if input.is_active(Down) {
            self.position -= Vector3::unit_y() * self.speed;
        }

        let delta = input.mouse_delta() * self.sensitivity;

        self.rotation.y += delta.x;
        self.rotation.x = clamp(self.rotation.x + delta.y, -90.0, 90.0);

        if input.scroll_wheel() > 0.5 {
            self.speed *= 1.1;
        }

        if input.scroll_wheel() < -0.5 {
            self.speed /= 1.1;
        }

        {
            let view = Matrix4::from_translation(self.position)
                * Matrix4::from_angle_y(Deg(self.rotation.y))
                * Matrix4::from_angle_x(Deg(self.rotation.x))
                    .invert()
                    .unwrap();

            *matrix = self.projection * view;
        }
    }

    pub fn resize_viewport(&mut self, width: u32, height: u32) {
        self.projection = perspective(
            Deg(self.fov),
            width as f32 / height as f32,
            self.near,
            self.far,
        );
    }

    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn right(&self) -> Vector3<f32> {
        Matrix3::from_angle_y(Deg(self.rotation.y))
            * Matrix3::from_angle_x(Deg(self.rotation.x))
            * Vector3::unit_x()
    }

    pub fn forward(&self) -> Vector3<f32> {
        Matrix3::from_angle_y(Deg(self.rotation.y))
            * Matrix3::from_angle_x(Deg(self.rotation.x))
            * -Vector3::unit_z()
    }
}
