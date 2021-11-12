mod camera;

use std::marker::PhantomData;
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{Input, Trigger},
    render::{data::LineVertex, GraphicsWorld},
};

use self::camera::Camera;

macro_rules! action {
    ($name:literal Key $elem:ident) => {
        ($name, Trigger::Key(VirtualKeyCode::$elem))
    };

    ($name:literal Btn $elem:ident) => {
        ($name, Trigger::Button(MouseButton::$elem))
    };
}

pub struct Editor<I, G> {
    camera: Camera,

    _i: PhantomData<I>,
    _g: PhantomData<G>,
}

impl<I, G> Editor<I, G>
where
    I: Input,
    G: GraphicsWorld,
{
    pub fn init(input: &mut I, gfx: &mut G) -> Self {
        #[rustfmt::skip]
        input.define_actions(&[
            action!( "movecam"  Btn Right ),
            action!( "forward"  Key W     ),
            action!( "backward" Key S     ),
            action!( "left"     Key A     ),
            action!( "right"    Key D     ),
            action!( "up"       Key E     ),
            action!( "down"     Key Q     ),
        ]);

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

        gfx.update_grid(10, 1.0);
        gfx.update_wireframe(&[A, B, B, C, C, A]);

        Self {
            camera: Camera::default(),
            _i: PhantomData,
            _g: PhantomData,
        }
    }

    pub fn process(&mut self, input: &I, gfx: &mut G) {
        self.camera.process(input, gfx);
    }
}
