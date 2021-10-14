mod camera;
mod grid;
mod input;
mod model;
mod transform;

use crate::input::ElementKind;
use camera::Camera;
use grid::Grid;
use input::InputMapper;
use tools::{
    app::{
        event::Event,
        input::{ButtonKind, KeyKind},
        App, MainLoop,
    },
    gfx::{Color, Graphics},
    math::{Vector2, Vector3, Zero},
};

pub struct Viewport {
    input_mapper: InputMapper,
    inner_logic: Option<InnerLogic>,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            input_mapper: Default::default(),
            inner_logic: None,
        }
    }
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {
        while let Some(event) = app.poll_event() {
            match event {
                Event::Initialized => {
                    self.input_mapper
                        .register_action("add_point", vec![ElementKind::Button(ButtonKind::Left)]);
                    self.input_mapper
                        .register_action("left", vec![ElementKind::Key(KeyKind::Left)]);
                    self.input_mapper
                        .register_action("right", vec![ElementKind::Key(KeyKind::Right)]);
                    self.input_mapper
                        .register_action("up", vec![ElementKind::Key(KeyKind::Up)]);
                    self.input_mapper
                        .register_action("down", vec![ElementKind::Key(KeyKind::Down)]);

                    self.inner_logic = Some(InnerLogic::new(app.graphics()));
                }
                Event::Resized(width, height) => {
                    if let Some(logic) = &mut self.inner_logic {
                        logic.resized(width, height);
                    }
                }
                Event::RawInput(input) => self.input_mapper.process_raw_input(input),
            };
        }

        if let Some(logic) = &mut self.inner_logic {
            logic.process(&self.input_mapper, app.graphics());
        }
    }
}

struct InnerLogic {
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
