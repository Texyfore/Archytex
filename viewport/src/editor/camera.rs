use cgmath::{num_traits::clamp, Deg, Matrix3, Matrix4, Vector2, Vector3, Zero};

use crate::{input::Input, render::GraphicsWorld};

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector2<f32>,
    speed: f32,
    sensitivity: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Vector2::zero(),
            speed: 0.1,
            sensitivity: 0.1,
        }
    }
}

impl Camera {
    pub fn process<I: Input, G: GraphicsWorld>(&mut self, input: &I, gfx: &mut G) {
        if !input.is_active("movecam") {
            return;
        }

        if input.is_active("forward") {
            self.position += self.forward() * self.speed;
        }

        if input.is_active("backward") {
            self.position -= self.forward() * self.speed;
        }

        if input.is_active("left") {
            self.position -= self.right() * self.speed;
        }

        if input.is_active("right") {
            self.position += self.right() * self.speed;
        }

        if input.is_active("up") {
            self.position += Vector3::unit_y() * self.speed;
        }

        if input.is_active("down") {
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

        gfx.update_camera_view(
            Matrix4::from_translation(self.position)
                * Matrix4::from_angle_y(Deg(self.rotation.y))
                * Matrix4::from_angle_x(Deg(self.rotation.x)),
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
