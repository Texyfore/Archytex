mod camera;

use cgmath::{Matrix4, SquareMatrix};
use std::{marker::PhantomData, rc::Rc};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{Input, Trigger},
    render::{
        data::BrushVertex, BrushCommand, BrushComponent, BrushMesh, GraphicsWorld, Texture,
        Transform,
    },
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
    brush: Rc<BrushMesh>,
    transform: Rc<Transform>,
    texture: Rc<Texture>,

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

        let brush = gfx.create_brush_mesh(
            &[
                BrushVertex {
                    position: [0.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                BrushVertex {
                    position: [1.0, 0.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    texcoord: [1.0, 0.0],
                },
                BrushVertex {
                    position: [1.0, 1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    texcoord: [1.0, 1.0],
                },
                BrushVertex {
                    position: [0.0, 1.0, 0.0],
                    normal: [0.0, 0.0, 0.0],
                    texcoord: [0.0, 1.0],
                },
            ],
            &[[0, 1, 2], [0, 2, 3]],
        );

        let transform = gfx.create_transform(Matrix4::identity());
        let texture =
            gfx.create_texture(&image::load_from_memory(include_bytes!("res/nodraw.png")).unwrap());

        Self {
            camera: Camera::default(),
            brush,
            transform,
            texture,
            _i: PhantomData,
            _g: PhantomData,
        }
    }

    pub fn process(&mut self, input: &I, gfx: &mut G) {
        self.camera.process(input, gfx);
        gfx.draw_brush(BrushCommand {
            transform: self.transform.clone(),
            components: vec![BrushComponent {
                mesh: self.brush.clone(),
                texture: self.texture.clone(),
            }],
        });
    }
}
