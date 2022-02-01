use cgmath::{Vector3, Zero};
use renderer::Renderer;
use winit::event::{MouseButton, VirtualKeyCode};

use crate::editor::{
    elements::{ElementKind, Movable, Solid, SolidID},
    graphics::{self, Graphics, MeshGenInput},
    scene::{Action, RaycastHit},
};

use super::{generic, Context, Tool};

#[derive(Default)]
pub struct Hub {
    regen: bool,
}

impl Tool for Hub {
    fn process(&mut self, ctx: &mut Context) {
        if !self.regen {
            ctx.set_regen();
            self.regen = true;
        }

        if ctx.input().was_button_down_once(MouseButton::Left) {
            ctx.switch_to(Box::new(generic::Select::<SelectProvider>::default()));
            return;
        }

        if ctx.input().is_key_down_once(VirtualKeyCode::G) {
            let mouse_pos = ctx.input().mouse_pos();
            let ray = ctx.camera().screen_ray(mouse_pos);
            let elements = ctx.scene().clone_and_hide_solids(ElementKind::Point);

            if let Some(tool) = generic::Move::<MoveProvider>::new(&ray, elements) {
                ctx.switch_to(Box::new(tool));
                return;
            }
        }

        self.process_undo_redo(ctx);
        self.process_camera(ctx);
    }

    fn element_mask(&self) -> ElementKind {
        ElementKind::Point
    }

    fn cancellable(&self) -> bool {
        true
    }
}

#[derive(Default)]
struct SelectProvider;

impl generic::SelectProvider for SelectProvider {
    fn deselect_action() -> Action {
        Action::DeselectPoints
    }

    fn select_action(hit: RaycastHit) -> Option<Action> {
        (!hit.points.is_empty()).then(|| Action::SelectPoints(hit.points))
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn element_mask() -> ElementKind {
        ElementKind::Point
    }
}

struct MoveProvider;

impl generic::MoveProvider for MoveProvider {
    type Element = (SolidID, Solid);

    fn center(elements: &[Self::Element]) -> Vector3<f32> {
        let mut center = Vector3::zero();

        for (_, solid) in elements {
            center += solid.center(ElementKind::Point);
        }

        center / elements.len() as f32
    }

    fn displace(elements: &mut [Self::Element], delta: Vector3<i32>) {
        for (_, solid) in elements {
            solid.displace(ElementKind::Point, delta);
        }
    }

    fn action(_elements: &[Self::Element], delta: Vector3<i32>) -> Action {
        Action::Move {
            kind: ElementKind::Point,
            delta,
        }
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn element_kind() -> ElementKind {
        ElementKind::Point
    }

    fn regen(renderer: &Renderer, elements: &[Self::Element], graphics: &mut Option<Graphics>) {
        graphics::generate(
            MeshGenInput {
                renderer,
                mask: ElementKind::Point,
                solids: elements.iter().map(|(_, solid)| solid),
            },
            graphics,
        )
    }
}