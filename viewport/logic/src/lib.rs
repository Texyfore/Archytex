mod input;

use input::InputMapper;
use tools::{
    app::{App, MainLoop},
    gfx::{Mesh, Tri, Vert},
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
        let mesh = Mesh::new(
            app.graphics(),
            &[
                Vert {
                    pos: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    uv: [0.0, 0.0],
                },
                Vert {
                    pos: [1.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    uv: [1.0, 0.0],
                },
                Vert {
                    pos: [1.0, 1.0, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    uv: [1.0, 1.0],
                },
                Vert {
                    pos: [0.0, 1.0, 0.0],
                    normal: [0.0, 0.0, 1.0],
                    uv: [0.0, 1.0],
                },
            ],
            &[Tri { idx: [0, 1, 2] }, Tri { idx: [0, 2, 3] }],
        );
        mesh.draw(app.graphics(), Mat4::translation(Vec3::new(0.0, 0.0, -2.0)));
    }
}
