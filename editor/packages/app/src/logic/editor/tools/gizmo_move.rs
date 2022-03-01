use cgmath::Vector3;
use winit::event::MouseButton;

use crate::{
    graphics::{Canvas, Graphics},
    logic::editor::{common::Axis, gizmo::ArrowGraphics},
};

use super::{CameraTool, Context, Tool};

pub struct GizmoMove {
    axis: Axis,
    graphics: ArrowGraphics,
}

impl GizmoMove {
    pub fn new(graphics: &Graphics, center: Vector3<f32>, axis: Axis) -> Self {
        let arrows = ArrowGraphics::new_empty(graphics);
        arrows.modify(graphics, center, Some(axis), true);

        Self {
            axis,
            graphics: arrows,
        }
    }
}

impl Tool for GizmoMove {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        if ctx.input.was_button_down_once(MouseButton::Left) {
            return Some(Box::new(CameraTool::new(ctx.graphics, false)));
        }

        None
    }

    fn render(&self, canvas: &mut Canvas) {
        self.graphics.render(canvas);
    }
}
