mod camera;
mod input;
mod model;

use crate::input::ElementKind;
use camera::Camera;
use input::InputMapper;
use tools::{
    app::{event::Event, input::ButtonKind, App, MainLoop},
    gfx::Graphics,
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
                    self.inner_logic = Some(InnerLogic::new());
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
}

impl InnerLogic {
    pub fn new() -> Self {
        Self {
            camera: Camera::new(60.0, 0.1, 100.0),
        }
    }

    pub fn resized(&mut self, width: u32, height: u32) {
        self.camera
            .calculate_projection(width as f32 / height as f32);
    }

    pub fn process(&mut self, gfx: &Graphics) {
        gfx.set_camera_view(self.camera.view);
        gfx.set_camera_projection(self.camera.projection);
    }
}
