mod camera;
mod grid;
mod input;
mod model;

use crate::input::ElementKind;
use camera::Camera;
use grid::Grid;
use input::InputMapper;
use tools::{app::{event::Event, input::ButtonKind, App, MainLoop}, gfx::{Color, Graphics}, math::{Mat4, Vec3}};

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
            logic.process(app.graphics());
        }
    }
}

struct InnerLogic {
    camera: Camera,
    grid: Grid,
}

impl InnerLogic {
    pub fn new(gfx: &Graphics) -> Self {
        Self {
            camera: Camera::new(60.0, 0.1, 100.0),
            grid: Grid::new(10, Color::new(0.5, 0.5, 0.5, 1.0), gfx),
        }
    }

    pub fn resized(&mut self, width: u32, height: u32) {
        self.camera
            .calculate_projection(width as f32 / height as f32);
    }

    pub fn process(&mut self, gfx: &Graphics) {
        self.camera.view = Mat4::from_translation(Vec3::new(0.0, 5.0, 10.0));

        gfx.set_camera_view(self.camera.view);
        gfx.set_camera_projection(self.camera.projection);
        self.grid.draw(gfx);
    }
}
