use std::marker::PhantomData;

use cgmath::{vec3, Matrix4, Rad};

use crate::{
    input::Input,
    render::{data::LineVertex, GraphicsWorld},
};

pub struct Editor<I, G> {
    a: f32,

    _i: PhantomData<I>,
    _g: PhantomData<G>,
}

impl<I, G> Editor<I, G>
where
    I: Input,
    G: GraphicsWorld,
{
    pub fn init(input: &mut I, gfx: &mut G) -> Self {
        input.define_actions(&[]);

        const A: LineVertex = LineVertex {
            position: [-0.5, -0.37, 0.0],
            color: [1.0, 0.0, 0.0, 1.0],
        };

        const B: LineVertex = LineVertex {
            position: [0.5, -0.37, 0.0],
            color: [0.0, 1.0, 0.0, 1.0],
        };

        const C: LineVertex = LineVertex {
            position: [0.0, 0.5, 0.0],
            color: [0.0, 0.0, 1.0, 1.0],
        };

        gfx.update_grid(5, 1.0);
        gfx.update_wireframe(&[A, B, B, C, C, A]);

        Self {
            a: 0.0,
            _i: PhantomData,
            _g: PhantomData,
        }
    }

    pub fn process(&mut self, input: &I, gfx: &mut G) {
        gfx.update_camera_view(
            Matrix4::from_angle_y(Rad(self.a)) * Matrix4::from_translation(vec3(0.0, 2.0, 5.0)),
        );
        self.a += 0.01;
    }
}
