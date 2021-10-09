mod input;
mod mesh;

use input::InputMapper;
use mesh::MeshBuilder;
use tools::{
    app::{event::Event, App, MainLoop},
    math::{Mat4, Vec3},
};

pub struct Viewport {
    input_mapper: InputMapper,
}

impl Default for Viewport {
    fn default() -> Self {
        let mut input_mapper = InputMapper::default();
        Self { input_mapper }
    }
}

impl MainLoop for Viewport {
    fn process(&mut self, app: &mut App) {
        while let Some(event) = app.poll_event() {
            match event {
                Event::Initialized => {}
                Event::RawInput(input) => self.input_mapper.process_raw_input(input),
            };
        }

        let mesh = {
            let mut builder = MeshBuilder::default();
            builder.push_quad(
                [
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                ],
                Vec3::new(0.0, 0.0, 1.0),
            );
            builder.build(app.graphics())
        };
        mesh.draw(app.graphics(), Mat4::translation(Vec3::new(0.0, 0.0, -2.0)));
    }
}
