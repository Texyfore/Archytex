use cgmath::{
    num_traits::clamp, perspective, vec2, vec3, Deg, Matrix3, Matrix4, SquareMatrix, Vector2,
    Vector3, Vector4, Zero,
};

use crate::{input::InputMapper, math::Ray, net, render::Scene};

use super::ActionBinding::*;

pub struct WorldCamera {
    position: Vector3<f32>,
    rotation: Vector2<f32>,
    speed: f32,
    sensitivity: f32,

    fov: f32,
    near: f32,
    far: f32,

    view: Matrix4<f32>,
    projection: Matrix4<f32>,
    viewport_size: Vector2<f32>,
}

impl Default for WorldCamera {
    fn default() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Vector2::zero(),
            speed: 8.0,
            sensitivity: 4.0,

            fov: 80.0,
            near: 0.1,
            far: 100.0,

            view: Matrix4::identity(),
            projection: Matrix4::identity(),
            viewport_size: Vector2::zero(),
        }
    }
}

impl WorldCamera {
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = 8.0 * 1.1f32.powf(speed - 50.0);
    }

    pub fn process(&mut self, dt: f32, input: &InputMapper) {
        if input.is_active(MoveCamera) {
            if input.is_active(Forward) {
                self.position += self.forward() * self.speed * dt;
            }

            if input.is_active(Backward) {
                self.position -= self.forward() * self.speed * dt;
            }

            if input.is_active(Left) {
                self.position -= self.right() * self.speed * dt;
            }

            if input.is_active(Right) {
                self.position += self.right() * self.speed * dt;
            }

            if input.is_active(Up) {
                self.position += Vector3::unit_y() * self.speed * dt;
            }

            if input.is_active(Down) {
                self.position -= Vector3::unit_y() * self.speed * dt;
            }

            let delta = input.mouse_delta() * self.sensitivity * dt;

            self.rotation.y += delta.x;
            self.rotation.x = clamp(self.rotation.x + delta.y, -90.0, 90.0);

            if input.scroll_wheel() > 0.5 {
                self.speed *= 1.1;
                net::send_packet(format!(
                    r#"{{ "message": "set-camera-speed", "speed": {} }}"#,
                    self.speed
                ));
            }

            if input.scroll_wheel() < -0.5 {
                self.speed /= 1.1;
                net::send_packet(format!(
                    r#"{{ "message": "set-camera-speed", "speed": {} }}"#,
                    self.speed
                ));
            }
        }

        self.view = Matrix4::from_translation(self.position)
            * Matrix4::from_angle_y(Deg(self.rotation.y))
            * Matrix4::from_angle_x(Deg(self.rotation.x));
    }

    pub fn render(&self, scene: &mut Scene) {
        scene.world_pass.camera_matrix = self.projection * self.view.invert().unwrap();
    }

    pub fn resize_viewport(&mut self, width: u32, height: u32) {
        self.viewport_size = vec2(width as f32, height as f32);
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

    pub fn screen_ray(&self, coords: Vector2<f32>) -> Ray {
        let coords = (vec2(
            coords.x / self.viewport_size.x,
            1.0 - coords.y / self.viewport_size.y,
        ) - vec2(0.5, 0.5))
            * 2.0;

        let unproject = self.view * self.projection.invert().unwrap();

        let a = unproject * Vector4::new(coords.x, coords.y, 0.0, 1.0);
        let b = unproject * Vector4::new(coords.x, coords.y, 1.0, 1.0);

        let a = vec3(a.x / a.w, a.y / a.w, a.z / a.w);
        let b = vec3(b.x / b.w, b.y / b.w, b.z / b.w);

        Ray { origin: a, end: b }
    }

    pub fn project(&self, point: Vector3<f32>, clip_displace: f32) -> Option<Vector3<f32>> {
        let view = self.view.invert().unwrap() * Vector4::new(point.x, point.y, point.z, 1.0);
        let clip = self.projection * view;
        let clip = vec3(clip.x, clip.y, clip.z + clip_displace) / clip.w;

        if (0.0..1.0).contains(&clip.z) {
            if (-1.0..=1.0).contains(&clip.x) && (-1.0..=1.0).contains(&clip.y) {
                let moved = vec2(clip.x + 1.0, 2.0 - (clip.y + 1.0)) * 0.5;
                Some(vec3(
                    moved.x * self.viewport_size.x,
                    moved.y * self.viewport_size.y,
                    clip.z,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct SpriteCamera {
    projection: Matrix4<f32>,
}

impl Default for SpriteCamera {
    fn default() -> Self {
        Self {
            projection: Matrix4::identity(),
        }
    }
}

impl SpriteCamera {
    pub fn render(&self, scene: &mut Scene) {
        scene.sprite_pass.camera_matrix = self.projection;
    }

    pub fn resize_viewport(&mut self, width: u32, height: u32) {
        self.projection = Matrix4::from_translation(vec3(-1.0, 1.0, 0.0))
            * Matrix4::from_nonuniform_scale(2.0 / width as f32, -2.0 / height as f32, 1.0);
    }
}
