use asset::scene;
use cgmath::{
    perspective, vec2, vec3, Deg, Matrix3, Matrix4, SquareMatrix, Transform, Vector2, Vector3,
    Vector4, Zero,
};

use crate::{graphics::structures::CameraMatrices, math::Ray};

pub struct Camera {
    position: Vector3<f32>,
    rotation: Vector2<f32>,
    projection: Matrix4<f32>,
    viewport_size: Vector2<f32>,
    speed: i32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: vec3(20.0, 20.0, 20.0),
            rotation: vec2(-45.0, 45.0),
            projection: Matrix4::identity(),
            viewport_size: Vector2::zero(),
            speed: 50,
        }
    }
}

impl Camera {
    pub fn recalc(&mut self, width: u32, height: u32) {
        let (width, height) = (width as f32, height as f32);
        self.projection = perspective(Deg(80.0), width / height, 0.05, 200.0);
        self.viewport_size = Vector2::new(width, height)
    }

    pub fn matrices(&self) -> CameraMatrices {
        let view_to_world = self.view_to_world();
        let world_to_clip = self.projection * view_to_world.inverse_transform().unwrap();

        CameraMatrices {
            world_to_clip,
            view_to_world,
        }
    }

    pub fn move_forward(&mut self, delta: f32) {
        self.position += self.forward() * self.speed_multiplier() * delta;
    }

    pub fn move_backward(&mut self, delta: f32) {
        self.position -= self.forward() * self.speed_multiplier() * delta;
    }

    pub fn move_right(&mut self, delta: f32) {
        self.position += self.right() * self.speed_multiplier() * delta;
    }

    pub fn move_left(&mut self, delta: f32) {
        self.position -= self.right() * self.speed_multiplier() * delta;
    }

    pub fn move_up(&mut self, delta: f32) {
        self.position += Vector3::unit_y() * self.speed_multiplier() * delta;
    }

    pub fn move_down(&mut self, delta: f32) {
        self.position -= Vector3::unit_y() * self.speed_multiplier() * delta;
    }

    pub fn look(&mut self, mouse_delta: Vector2<f32>, delta: f32) {
        const SENSITIVITY: f32 = 6.0;
        self.rotation.y -= mouse_delta.x * SENSITIVITY * delta;
        self.rotation.x =
            (self.rotation.x - mouse_delta.y * SENSITIVITY * delta).clamp(-90.0, 90.0);
    }

    pub fn increase_speed(&mut self) {
        self.speed += 1;
    }

    pub fn decrease_speed(&mut self) {
        self.speed -= 1;
    }

    pub fn screen_ray(&self, coords: Vector2<f32>) -> Ray {
        let coords = (vec2(
            coords.x / self.viewport_size.x,
            1.0 - coords.y / self.viewport_size.y,
        ) - vec2(0.5, 0.5))
            * 2.0;

        let unproject = self.view_to_world() * self.projection.invert().unwrap();

        let a = unproject * Vector4::new(coords.x, coords.y, 0.0, 1.0);
        let b = unproject * Vector4::new(coords.x, coords.y, 1.0, 1.0);

        let a = vec3(a.x / a.w, a.y / a.w, a.z / a.w);
        let b = vec3(b.x / b.w, b.y / b.w, b.z / b.w);

        Ray { start: a, end: b }
    }

    pub fn project(&self, point: Vector3<f32>) -> Option<Vector3<f32>> {
        let point = point.extend(1.0);
        let projected = self.projection * self.view_to_world().inverse_transform().unwrap() * point;

        if projected.w.abs() > 0.00001 {
            let clip = projected.truncate() / projected.w;
            if (0.0..1.0).contains(&clip.z)
                && (-1.0..=1.0).contains(&clip.x)
                && (-1.0..=1.0).contains(&clip.y)
            {
                let moved = vec2(clip.x + 1.0, 2.0 - (clip.y + 1.0)) * 0.5;
                return Some(vec3(
                    moved.x * self.viewport_size.x,
                    moved.y * self.viewport_size.y,
                    clip.z,
                ));
            }
        }

        None
    }

    pub fn save(&self) -> scene::Camera {
        scene::Camera {
            position: self.position,
            rotation: self.rotation,
        }
    }

    pub fn load(&mut self, camera: &scene::Camera) {
        self.position = camera.position;
        self.rotation = camera.rotation;
    }

    fn forward(&self) -> Vector3<f32> {
        Matrix3::from_angle_y(Deg(self.rotation.y))
            * Matrix3::from_angle_x(Deg(self.rotation.x))
            * -Vector3::unit_z()
    }

    fn right(&self) -> Vector3<f32> {
        Matrix3::from_angle_y(Deg(self.rotation.y))
            * Matrix3::from_angle_x(Deg(self.rotation.x))
            * Vector3::unit_x()
    }

    fn speed_multiplier(&self) -> f32 {
        8.0 * 1.1f32.powi(self.speed - 50)
    }

    fn view_to_world(&self) -> Matrix4<f32> {
        Matrix4::from_translation(self.position)
            * Matrix4::from_angle_y(Deg(self.rotation.y))
            * Matrix4::from_angle_x(Deg(self.rotation.x))
    }
}

// TODO: Tests
