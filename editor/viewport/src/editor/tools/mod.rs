use winit::event::VirtualKeyCode;

use crate::input::Input;

use super::{
    camera::Camera,
    scene::{GraphicsMask, Scene},
};

mod solid;

pub(super) struct ToolManager {
    tool: Box<dyn Tool>,
}

impl Default for ToolManager {
    fn default() -> Self {
        Self {
            tool: Box::new(solid::Select::default()),
        }
    }
}

impl ToolManager {
    pub fn process(&mut self, mut ctx: OuterContext) -> Output {
        let mut regen = false;

        if ctx.input.is_key_down_once(VirtualKeyCode::Key1) {
            self.tool.cancelled(&mut ctx);
            self.tool = Box::new(solid::Select::default());
            regen = true;
        }

        let output = self.tool.process(&mut ctx);
        if let Some(switch_to) = output.switch_to {
            self.tool.cancelled(&mut ctx);
            self.tool = switch_to;
            regen = true;
        }

        Output {
            can_move: output.can_move,
            regen: output.regen || regen,
        }
    }

    pub fn graphics_mask(&self) -> GraphicsMask {
        self.tool.graphics_mask()
    }
}

pub(super) struct OuterContext<'a> {
    pub input: &'a Input,
    pub camera: &'a Camera,
    pub scene: &'a mut Scene,
}

pub(super) struct Output {
    pub can_move: bool,
    pub regen: bool,
}

trait Tool {
    fn process(&mut self, ctx: &mut OuterContext) -> ToolOutput;
    fn cancelled(&mut self, ctx: &mut OuterContext);
    fn graphics_mask(&self) -> GraphicsMask;
}

struct ToolOutput {
    switch_to: Option<Box<dyn Tool>>,
    can_move: bool,
    regen: bool,
}
