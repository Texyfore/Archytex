mod brush;
mod camera;
mod config;
mod texture;

use std::marker::PhantomData;
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{Input, Trigger},
    render::GraphicsWorld,
};

use self::{brush::BrushBank, camera::Camera, texture::TextureBank, ActionBinding::*};

macro_rules! action {
    ($name:ident Key $elem:ident) => {
        (ActionBinding::$name, Trigger::Key(VirtualKeyCode::$elem))
    };

    ($name:ident Btn $elem:ident) => {
        (ActionBinding::$name, Trigger::Button(MouseButton::$elem))
    };
}

macro_rules! actions {
    ($($name:ident $ty:ident $elem:ident,)*) => {
        #[derive(Clone, Eq, PartialEq, Hash)]
        pub enum ActionBinding {
            $($name,)*
        }

        const ACTION_DEFINITIONS: &[(ActionBinding, Trigger)] = &[
            $(action!($name $ty $elem),)*
        ];
    };
}

actions! {
    // Camera movement ////////////////

    EnableCameraMovement Btn Right    ,
    Forward              Key W        ,
    Backward             Key S        ,
    Left                 Key A        ,
    Right                Key D        ,
    Up                   Key E        ,
    Down                 Key Q        ,

    // Selection //////////////////////

    EnableMultiSelect    Key LShift   ,
    Select               Btn Left     ,
    Deselect             Key X        ,

    // Brush manipulation /////////////

    EnableAddBrush       Key LControl ,
    AddBrush             Btn Left     ,
    DeleteBrush          Key Delete   ,

    // Debug //////////////////////////

    Shift                Key LShift   ,
    Control              Key LControl ,
    Inc                  Key Up       ,
    Dec                  Key Down     ,
    BrushMode            Key B        ,
    FaceMode             Key F        ,
    VertexMode           Key V        ,

    ///////////////////////////////////
}

pub struct Editor<I, G> {
    mode: EditMode,
    camera: Camera,
    texture_bank: TextureBank,
    brush_bank: BrushBank,

    _i: PhantomData<I>,
    _g: PhantomData<G>,
}

impl<I, G> Editor<I, G>
where
    I: Input,
    G: GraphicsWorld,
{
    pub fn init(input: &mut I, gfx: &mut G) -> Self {
        input.define_actions(ACTION_DEFINITIONS);
        gfx.update_grid(10, 1.0);

        Self {
            mode: EditMode::Brush,
            camera: Camera::default(),
            texture_bank: Default::default(),
            brush_bank: Default::default(),
            _i: PhantomData,
            _g: PhantomData,
        }
    }

    pub fn add_texture(&mut self, gfx: &G, uuid: u64, bytes: &[u8]) {
        self.texture_bank.add(gfx, uuid, bytes);
    }

    pub fn process(&mut self, input: &I, gfx: &mut G) {
        if input.is_active(Control) {
            if input.is_active_once(BrushMode) {
                self.mode = EditMode::Brush;
            } else if input.is_active_once(FaceMode) {
                self.mode = EditMode::Face;
            } else if input.is_active_once(VertexMode) {
                self.mode = EditMode::Vertex;
            }
        }

        self.camera.process(input, gfx);
        self.brush_bank
            .process(input, gfx, &self.texture_bank, &self.mode);
    }
}

pub enum EditMode {
    Brush,
    Face,
    Vertex,
}
