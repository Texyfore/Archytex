mod brush;
mod camera;

use cgmath::vec3;
use std::{marker::PhantomData, rc::Rc};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{Input, Trigger},
    render::{GraphicsWorld, Texture},
};

use self::{brush::Brush, camera::Camera};

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
    texture: Rc<Texture>,
    brush: Brush,

    _i: PhantomData<I>,
    _g: PhantomData<G>,
}

impl<I, G> Editor<I, G>
where
    I: Input,
    G: GraphicsWorld,
{
    pub fn init(input: &mut I, gfx: &mut G) -> Self {
        input.define_actions(actions!(
            "movecam"  Btn Right,
            "forward"  Key W    ,
            "backward" Key S    ,
            "left"     Key A    ,
            "right"    Key D    ,
            "up"       Key E    ,
            "down"     Key Q    ,
        ));

        gfx.update_grid(10, 1.0);

        let mut brush = Brush::new(
            gfx,
            vec![
                vec3(0.0, 0.0, 0.0),
                vec3(1.0, 0.0, 0.0),
                vec3(1.0, 0.0, 1.0),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 1.0, 0.0),
                vec3(1.0, 1.0, 0.0),
                vec3(1.0, 1.0, 1.0),
                vec3(0.0, 1.0, 1.0),
            ],
            vec![
                [0, 1, 2, 3],
                [7, 6, 5, 4],
                [4, 5, 1, 0],
                [6, 7, 3, 2],
                [0, 3, 7, 4],
                [5, 6, 2, 1],
            ],
        );
        brush.regenerate(gfx);

        let texture =
            gfx.create_texture(&image::load_from_memory(include_bytes!("res/nodraw.png")).unwrap());

        Self {
            camera: Camera::default(),
            brush,
            texture,
            _i: PhantomData,
            _g: PhantomData,
        }
    }

    pub fn process(&mut self, input: &I, gfx: &mut G) {
        self.camera.process(input, gfx);
        self.brush.draw(gfx, self.texture.clone());
    }
}
