use cgmath::vec3;
use winit::event::MouseButton;

use crate::editor::{
    graphics::GraphicsMask,
    scene::{Action, RaycastEndpointKind, RaycastHit, Solid},
};

use super::{generic, Context, Tool};

#[derive(Default)]
pub struct Hub;

impl Tool for Hub {
    fn process(&mut self, ctx: &mut Context) {
        if ctx.input().was_button_down_once(MouseButton::Left) {
            ctx.switch_to(Box::new(generic::Select::<SelectProvider>::default()));
        }

        if ctx.input().is_button_down_once(MouseButton::Middle) {
            ctx.scene().act(Action::AddSolid(Solid::new(
                vec3(0, 0, 0),
                vec3(100, 100, 100),
            )));
            ctx.set_regen();
        }

        self.process_undo_redo(ctx);
        self.process_camera(ctx);
    }

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}

#[derive(Default)]
struct SelectProvider;

impl generic::SelectProvider for SelectProvider {
    fn deselect_action() -> Action {
        Action::DeselectSolids
    }

    fn select_action(hit: RaycastHit) -> Option<Action> {
        if let RaycastEndpointKind::Face { solid_id, .. } = hit.endpoint.kind {
            Some(Action::SelectSolids(vec![solid_id]))
        } else {
            None
        }
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn graphics_mask() -> GraphicsMask {
        GraphicsMask::Solids
    }
}
