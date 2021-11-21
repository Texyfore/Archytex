mod brush;
mod camera;
mod config;

use std::rc::Rc;

use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    input::{InputMapper, Trigger},
    render::{LineBatch, LineFactory, LineVertex, Scene, SolidFactory},
};

use self::{
    brush::BrushBank,
    camera::{SpriteCamera, WorldCamera},
    ActionBinding::*,
};

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
    solid_factory: SolidFactory,
    line_factory: LineFactory,
    world_camera: WorldCamera,
    sprite_camera: SpriteCamera,
    brush_bank: BrushBank,
    mode: EditMode,
    grid: Rc<LineBatch>,
}

impl Editor {
    pub fn init(
        solid_factory: SolidFactory,
        line_factory: LineFactory,
        input: &mut InputMapper,
    ) -> Self {
        input.define_actions(ACTION_DEFINITIONS);

        let grid = line_factory.create(&generate_grid(10, 1.0));

        Self {
            solid_factory,
            line_factory,
            world_camera: Default::default(),
            sprite_camera: Default::default(),
            brush_bank: Default::default(),
            mode: EditMode::Brush,
            grid,
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

        self.world_camera
            .process(input, &mut scene.world_pass.camera_matrix);

        self.sprite_camera
            .process(&mut scene.sprite_pass.camera_matrix);

        self.brush_bank
            .process(&input, &mut scene.world_pass, &self.solid_factory);

        scene.world_pass.line_batches.push(self.grid.clone());
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.world_camera.resize_viewport(width, height);
        self.sprite_camera.resize_viewport(width, height);
    }
}

pub enum EditMode {
    Brush,
    Face,
    Vertex,
}

fn generate_grid(cell_count: i32, cell_size: f32) -> Vec<LineVertex> {
    let half_line_len = cell_count as f32 * cell_size;
    let gray = [0.1, 0.1, 0.1, 1.0];
    let red = [0.4, 0.1, 0.1, 1.0];
    let blue = [0.1, 0.1, 0.4, 1.0];

    let mut vertices = Vec::with_capacity(cell_count as usize * 8 + 4);

    vertices.push(LineVertex {
        position: [-half_line_len, 0.0, 0.0],
        color: red,
    });

    vertices.push(LineVertex {
        position: [half_line_len, 0.0, 0.0],
        color: red,
    });

    vertices.push(LineVertex {
        position: [0.0, 0.0, -half_line_len],
        color: blue,
    });

    vertices.push(LineVertex {
        position: [0.0, 0.0, half_line_len],
        color: blue,
    });

    for sign in [-1.0, 1.0] {
        for i in 1..=cell_count {
            let pos = i as f32 * cell_size * sign;

            vertices.push(LineVertex {
                position: [-half_line_len, 0.0, pos],
                color: gray,
            });

            vertices.push(LineVertex {
                position: [half_line_len, 0.0, pos],
                color: gray,
            });

            vertices.push(LineVertex {
                position: [pos, 0.0, -half_line_len],
                color: gray,
            });

            vertices.push(LineVertex {
                position: [pos, 0.0, half_line_len],
                color: gray,
            });
        }
    }

    vertices
}
