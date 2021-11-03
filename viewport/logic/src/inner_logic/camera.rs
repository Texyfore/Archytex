use super::transform::Transform;
use crate::input::InputMapper;
use tools::{
    gfx::Graphics,
    math::{num_traits::clamp, perspective, Deg, Matrix4, SquareMatrix, Vector3},
};

pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub near: f32,
    pub far: f32,
    pub projection: Matrix4<f32>,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Camera {
    pub fn new(fov: f32, near: f32, far: f32) -> Self {
        Self {
            transform: Transform::identity(),
            fov,
            near,
            far,
            projection: Matrix4::identity(),
            speed: 0.1,
            sensitivity: 0.4,
        }
    }

    pub fn calculate_projection(&mut self, aspect: f32) {
        self.projection = perspective(Deg(self.fov), aspect, self.near, self.far)
    }

    pub fn update(&mut self, input: &InputMapper, gfx: &Graphics) {
        if !input.query_action("look") {
            // We only really need to update the camera when the user is
            // in "look mode".
            return;
        }

        if input.query_action("forward") {
            self.transform.position += self.transform.forward() * self.speed;
        }

        if input.query_action("backward") {
            self.transform.position -= self.transform.forward() * self.speed;
        }

        if input.query_action("left") {
            self.transform.position -= self.transform.right() * self.speed;
        }

        if input.query_action("right") {
            self.transform.position += self.transform.right() * self.speed;
        }

        if input.query_action("up") {
            self.transform.position += Vector3::unit_y() * self.speed;
        }

        if input.query_action("down") {
            self.transform.position -= Vector3::unit_y() * self.speed;
        }

        let delta = input.query_mouse_delta();

        self.transform
            .rotate(Vector3::new(-delta.y, -delta.x, 0.0) * self.sensitivity);

        self.transform.rotation.x = clamp(self.transform.rotation.x, -90.0, 90.0);

        if input.query_wheel_delta() > 0.1 {
            self.speed *= 1.1;
        }

        if input.query_wheel_delta() < -0.1 {
            self.speed /= 1.1;
        }

        gfx.set_camera_view(self.transform.calculate_matrix());
        gfx.set_camera_projection(self.projection);
    }
}
