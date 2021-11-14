mod brush;
mod camera;

use cgmath::{vec3, Matrix4, SquareMatrix};
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

        let mut brush = Brush::new(gfx, vec3(1.0, 1.0, 1.0), Matrix4::identity());
        brush.regenerate(gfx);

        let texture =
            gfx.create_texture(&image::load_from_memory(include_bytes!("res/nodraw.png")).unwrap());

        Self {
            camera: Camera::default(),
            brush,
            texture,
            a: 1.0,
            _i: PhantomData,
            _g: PhantomData,
        }
    }

    pub fn process(&mut self, input: &I, gfx: &mut G) {
        self.brush.set_point(2, vec3(1.0, 0.0, self.a));
        self.brush.set_point(3, vec3(0.0, 0.0, self.a));
        self.brush.regenerate(gfx);
        self.a += 0.001;

        self.camera.process(input, gfx);
        self.brush.draw(gfx, self.texture.clone());
    }
}
