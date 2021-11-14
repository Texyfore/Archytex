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

macro_rules! actions {
    ($($name:literal $ty:ident $elem:ident,)*) => {
        &[
            $(action!($name $ty $elem),)*
        ]
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
        input.define_actions(
            actions!(
                "movecam"  Btn Right,
                "forward"  Key W    ,
                "backward" Key S    ,
                "left"     Key A    ,
                "right"    Key D    ,
                "up"       Key E    ,
                "down"     Key Q    ,
            )
        );

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
