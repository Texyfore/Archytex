mod camera;
mod grid;
mod model;
mod transform;

use crate::input::InputMapper;
use camera::Camera;
use grid::Grid;
use tools::{
    gfx::{Color, Graphics},
    math::{Vector2, Vector3, Zero},
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

    pub fn process(&mut self, input: &InputMapper, gfx: &Graphics) {
        let mouse = {
            let (x, y) = input.query_mouse_pos();
            Vector2::new(x, y)
        };

        let delta = mouse - self.mouse_before;

        const SPEED: f32 = 0.05;

        if input.query_action("left") {
            self.camera.transform.position += Vector3::new(-1.0, 0.0, 0.0) * SPEED;
        }
        if input.query_action("right") {
            self.camera.transform.position += Vector3::new(1.0, 0.0, 0.0) * SPEED;
        }
        if input.query_action("up") {
            self.camera.transform.position += Vector3::new(0.0, 0.0, -1.0) * SPEED;
        }
        if input.query_action("down") {
            self.camera.transform.position += Vector3::new(0.0, 0.0, 1.0) * SPEED;
        }

        self.mouse_before = mouse;

        gfx.set_camera_view(self.camera.transform.calculate_matrix());
        gfx.set_camera_projection(self.camera.projection);
        self.grid.draw(gfx);
    }
}
