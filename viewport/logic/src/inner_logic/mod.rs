mod camera;
mod grid;
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
        self.camera.update(input, app.graphics());
        self.grid.draw(app.graphics());
    }
}
