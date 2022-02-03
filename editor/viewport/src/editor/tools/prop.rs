use asset_id::PropID;
use cgmath::{Vector3, Zero};
use winit::event::MouseButton;

use crate::editor::{
    elements::{ElementKind, Prop},
    scene::{Action, RaycastEndpointKind, RaycastHit},
};

use super::{generic, Context, Tool};

#[derive(Default)]
pub struct Hub;

impl Tool for Hub {
    fn process(&mut self, ctx: &mut Context) {
        if ctx.input().was_button_down_once(MouseButton::Left) {
            ctx.switch_to(Box::new(generic::Select::<SelectProvider>::default()));
            return;
        }

        if ctx.input().is_button_down_once(MouseButton::Right) {
            ctx.scene_mut().act(Action::NewProps(vec![Prop::new(
                PropID(0),
                Vector3::zero(),
            )]));
            ctx.set_regen();
        }
    }

    fn cancellable(&self) -> bool {
        true
    }

    fn element_mask(&self) -> ElementKind {
        ElementKind::Prop
    }
}

#[derive(Default)]
struct SelectProvider;

impl generic::SelectProvider for SelectProvider {
    fn deselect_action() -> Action {
        Action::DeselectProps
    }

    fn select_action(hit: RaycastHit) -> Option<Action> {
        hit.endpoint.and_then(|endpoint| match endpoint.kind {
            RaycastEndpointKind::Prop(id) => Some(Action::SelectProps(vec![id])),
            _ => None,
        })
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn element_mask() -> ElementKind {
        ElementKind::Prop
    }
}
