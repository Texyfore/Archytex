mod input;
mod mesh;

use input::InputMapper;
use mesh::MeshBuilder;
use tools::{
    app::{event::Event, App, MainLoop},
    gfx::{Image, Texture},
    math::{Mat4, Vec3},
};

pub struct Viewport {
    input_mapper: InputMapper,
    a: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        let mut input_mapper = InputMapper::default();
        Self {
            input_mapper,
            a: 0.0,
        }
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
            // Z-
            builder.push_quad(
                [
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                ],
                Vec3::new(0.0, 0.0, -1.0),
            );
            // Z+
            builder.push_quad(
                [
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec3::new(0.0, 1.0, 1.0),
                ],
                Vec3::new(0.0, 0.0, 1.0),
            );
            // X-
            builder.push_quad(
                [
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                ],
                Vec3::new(-1.0, 0.0, 0.0),
            );
            // X+
            builder.push_quad(
                [
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec3::new(1.0, 0.0, 1.0),
                ],
                Vec3::new(1.0, 0.0, 0.0),
            );
            // Y-
            builder.push_quad(
                [
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 0.0),
                    Vec3::new(1.0, 0.0, 1.0),
                    Vec3::new(0.0, 0.0, 1.0),
                ],
                Vec3::new(0.0, -1.0, 0.0),
            );
            // Y3
            builder.push_quad(
                [
                    Vec3::new(0.0, 1.0, 1.0),
                    Vec3::new(1.0, 1.0, 1.0),
                    Vec3::new(1.0, 1.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                ],
                Vec3::new(0.0, 1.0, 0.0),
            );

            builder.build(app.graphics())
        };

        let texture = Texture::new(app.graphics(), &Image::load(include_bytes!("dummy.png")));

        mesh.draw(
            app.graphics(),
            Mat4::translation(Vec3::fill(-0.5))
                * Mat4::rotation(Vec3::new(0.0, self.a, 0.0))
                * Mat4::translation(Vec3::new(0.0, (self.a * 0.5).sin() * 1.5, -5.0)),
            &texture,
        );

        self.a += 0.025;
    }
}
