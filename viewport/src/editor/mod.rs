mod camera;

use std::marker::PhantomData;
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{Input, Trigger},
    render::GraphicsWorld,
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

        gfx.update_grid(10, 1.0);

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
