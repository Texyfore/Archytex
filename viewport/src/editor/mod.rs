mod camera;
mod config;

use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{InputMapper, Trigger},
    render::Scene,
};

use self::{camera::Camera, ActionBinding::*};

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

pub struct Editor {
    mode: EditMode,
    camera: Camera,
}

impl Editor {
    pub fn init(input: &mut InputMapper) -> Self {
        input.define_actions(ACTION_DEFINITIONS);

        Self {
            mode: EditMode::Brush,
            camera: Camera::default(),
        }
    }

    pub fn process(&mut self, input: &InputMapper, scene: &mut Scene) {
        if input.is_active(Control) {
            if input.is_active_once(BrushMode) {
                self.mode = EditMode::Brush;
            } else if input.is_active_once(FaceMode) {
                self.mode = EditMode::Face;
            } else if input.is_active_once(VertexMode) {
                self.mode = EditMode::Vertex;
            }
        }

        self.camera
            .process(input, &mut scene.world_pass.camera_matrix);
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.camera.resize_viewport(width, height);
    }
}

pub enum EditMode {
    Brush,
    Face,
    Vertex,
}
