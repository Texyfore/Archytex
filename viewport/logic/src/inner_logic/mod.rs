mod camera;
mod grid;
mod model;
mod transform;

use crate::input::InputMapper;
use camera::Camera;
use grid::Grid;
use tools::{
    app::App,
    gfx::{Color, Graphics},
    math::{num_traits::clamp, Vector2, Vector3, Zero},
};

pub struct InnerLogic {
    camera: Camera,
    grid: Grid,
    mouse_before: Vector2<f32>,
}

impl InnerLogic {
    pub fn new(gfx: &Graphics) -> Self {
        let mut camera = Camera::new(60.0, 0.1, 100.0);
        camera.transform.position = Vector3::new(0.0, 5.0, 20.0);

        Self {
            camera,
            grid: Grid::new(10, Color::new(0.5, 0.5, 0.5, 1.0), gfx),
            mouse_before: Vector2::zero(),
        }
    }

    pub fn resized(&mut self, width: u32, height: u32) {
        self.camera
            .calculate_projection(width as f32 / height as f32);
    }

    pub fn process(&mut self, input: &InputMapper, app: &mut App) {
        let mouse = {
            let (x, y) = input.query_mouse_pos();
            Vector2::new(x, y)
        };

        let delta = mouse - self.mouse_before;

        if input.query_action("look") {
            if input.query_action("left") {
                self.camera.transform.position -= self.camera.transform.right() * self.camera.speed;
            }

            if input.query_action("right") {
                self.camera.transform.position += self.camera.transform.right() * self.camera.speed;
            }

            if input.query_action("forward") {
                self.camera.transform.position +=
                    self.camera.transform.forward() * self.camera.speed;
            }

            if input.query_action("backward") {
                self.camera.transform.position -=
                    self.camera.transform.forward() * self.camera.speed;
            }

            if input.query_action("up") {
                self.camera.transform.position += Vector3::unit_y() * self.camera.speed;
            }

            if input.query_action("down") {
                self.camera.transform.position -= Vector3::unit_y() * self.camera.speed;
            }

            const SENSITIVITY: f32 = 0.4;

            self.camera
                .transform
                .rotate(Vector3::new(-delta.y, -delta.x, 0.0) * SENSITIVITY);
            self.camera.transform.rotation.x = clamp(self.camera.transform.rotation.x, -90.0, 90.0);

            if input.query_wheel_delta() > 0.1 {
                self.camera.speed *= 1.1;
            } else if input.query_wheel_delta() < -0.1 {
                self.camera.speed /= 1.1;
            }
            app.set_cursor_visible(false);
        } else {
            app.set_cursor_visible(true);
        }

        self.mouse_before = mouse;

        app.graphics()
            .set_camera_view(self.camera.transform.calculate_matrix());
        app.graphics().set_camera_projection(self.camera.projection);
        self.grid.draw(app.graphics());
    }
}
