use std::iter::once;

use asset_id::GizmoID;
use cgmath::{MetricSpace, Vector2, Vector3, Zero};
use renderer::{
    scene::{GizmoObject, Scene},
    Renderer,
};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    editor::{
        elements::{ElementKind, Movable, Solid, SolidID},
        graphics::{self, Graphics, MeshGenInput},
        scene::{Action, RaycastEndpointKind, RaycastHit},
    },
    math::{MinMax, Snap},
};

use super::{generic, Context, Tool};

#[derive(Default)]
pub struct Hub {
    regen: bool,
    last_click_pos: Option<Vector2<f32>>,
}

impl Tool for Hub {
    fn process(&mut self, ctx: &mut Context) {
        if !self.regen {
            ctx.set_regen();
            self.regen = true;
        }

        if ctx.input().is_button_down_once(MouseButton::Left) {
            self.last_click_pos = Some(ctx.input().mouse_pos());
        }

        if let Some(last_click_pos) = self.last_click_pos {
            if ctx.input().mouse_pos().distance2(last_click_pos) > 100.0 {
                let hit = ctx.scene().raycast(ctx.input().mouse_pos(), ctx.camera());
                if let Some(endpoint) = hit.endpoint {
                    ctx.scene_mut().act(Action::DeselectSolids);
                    ctx.set_regen();
                    ctx.switch_to(Box::new(Add::new(
                        endpoint.point + endpoint.normal * 0.0001,
                    )));
                    return;
                }
            }
        }

        if ctx.input().was_button_down_once(MouseButton::Left) {
            self.last_click_pos = None;
            ctx.switch_to(Box::new(generic::Select::<SelectProvider>::default()));
            return;
        }

        if ctx.input().is_key_down_once(VirtualKeyCode::G) {
            let mouse_pos = ctx.input().mouse_pos();
            let ray = ctx.camera().screen_ray(mouse_pos);
            let elements = ctx.scene_mut().clone_and_hide_solids(ElementKind::Solid);

            if let Some(tool) = generic::Move::<MoveProvider>::new(&ray, elements) {
                ctx.switch_to(Box::new(tool));
                return;
            }
        }

        if ctx.input().is_key_down_once(VirtualKeyCode::C) {
            let mouse_pos = ctx.input().mouse_pos();
            let ray = ctx.camera().screen_ray(mouse_pos);
            let elements = ctx.scene_mut().clone_solids();

            if let Some(tool) = generic::Move::<CloneProvider>::new(&ray, elements) {
                ctx.switch_to(Box::new(tool));
                return;
            }
        }

        if ctx.input().is_key_down_once(VirtualKeyCode::Delete) {
            ctx.switch_to(Box::new(generic::Delete::<DeleteProvider>::default()));
            return;
        }

        self.process_undo_redo(ctx);
        self.process_camera(ctx);
    }

    fn element_mask(&self) -> ElementKind {
        ElementKind::Solid
    }

    fn cancellable(&self) -> bool {
        true
    }
}

struct Add {
    start: Vector3<f32>,
    graphics: Option<Graphics>,
}

impl Add {
    fn new(start: Vector3<f32>) -> Self {
        Self {
            start,
            graphics: None,
        }
    }
}

impl Tool for Add {
    fn process(&mut self, ctx: &mut Context) {
        let hit = ctx.scene().raycast(ctx.input().mouse_pos(), ctx.camera());
        if let Some(endpoint) = hit.endpoint {
            let end = endpoint.point + endpoint.normal * 0.0001;

            let start = self.start.snap(100);
            let end = end.snap(100);

            let min = start.min(end);
            let max = start.max(end).map(|e| e + 100);

            let solid = Solid::new(min, max - min);

            graphics::generate(
                MeshGenInput {
                    renderer: ctx.renderer(),
                    mask: ElementKind::Solid,
                    solids: once(&solid),
                },
                &mut self.graphics,
            );

            if ctx.input().was_button_down_once(MouseButton::Left) {
                ctx.scene_mut().act(Action::NewSolids(vec![solid]));
                ctx.set_regen();
                ctx.switch_to(Box::new(Hub::default()));
            }
        } else {
            self.graphics.take();
            if !ctx.input().is_button_down(MouseButton::Left) {
                ctx.switch_to(Box::new(Hub::default()));
            }
        }
    }

    fn render(&self, scene: &mut Scene) {
        if let Some(graphics) = &self.graphics {
            for solid_object in &graphics.solid_objects {
                scene.push_solid_object(solid_object.clone());
            }

            scene.push_line_object(graphics.line_object.clone());
            scene.push_gizmo_object(GizmoObject {
                id: GizmoID(0),
                instances: graphics.point_gizmos.clone(),
            });
        }
    }

    fn element_mask(&self) -> ElementKind {
        ElementKind::Solid
    }
}

#[derive(Default)]
struct SelectProvider;

impl generic::SelectProvider for SelectProvider {
    fn deselect_action() -> Action {
        Action::DeselectSolids
    }

    fn select_action(hit: RaycastHit) -> Option<Action> {
        if let Some(endpoint) = hit.endpoint {
            if let RaycastEndpointKind::Face { solid_id, .. } = endpoint.kind {
                return Some(Action::SelectSolids(vec![solid_id]));
            }
        }

        None
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn element_mask() -> ElementKind {
        ElementKind::Solid
    }
}

#[derive(Default)]
struct DeleteProvider;

impl generic::DeleteProvider for DeleteProvider {
    fn action() -> Action {
        Action::RemoveSelectedSolids
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn element_mask() -> ElementKind {
        ElementKind::Solid
    }
}

struct MoveProvider;

impl generic::MoveProvider for MoveProvider {
    type Element = (SolidID, Solid);

    fn center(elements: &[Self::Element]) -> Vector3<f32> {
        let mut center = Vector3::zero();

        for (_, solid) in elements {
            center += solid.center(ElementKind::Solid);
        }

        center / elements.len() as f32
    }

    fn displace(elements: &mut [Self::Element], delta: Vector3<i32>) {
        for (_, solid) in elements {
            solid.displace(ElementKind::Solid, delta);
        }
    }

    fn action(_elements: &[Self::Element], delta: Vector3<i32>) -> Action {
        Action::Move {
            kind: ElementKind::Solid,
            delta,
        }
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn element_kind() -> ElementKind {
        ElementKind::Solid
    }

    fn regen(renderer: &Renderer, elements: &[Self::Element], graphics: &mut Option<Graphics>) {
        graphics::generate(
            MeshGenInput {
                renderer,
                mask: ElementKind::Solid,
                solids: elements.iter().map(|(_, solid)| solid),
            },
            graphics,
        );
    }
}

struct CloneProvider;

impl generic::MoveProvider for CloneProvider {
    type Element = Solid;

    fn center(elements: &[Self::Element]) -> Vector3<f32> {
        let mut center = Vector3::zero();

        for solid in elements {
            center += solid.center(ElementKind::Solid);
        }

        center / elements.len() as f32
    }

    fn displace(elements: &mut [Self::Element], delta: Vector3<i32>) {
        for solid in elements {
            solid.displace(ElementKind::Solid, delta);
        }
    }

    fn action(elements: &[Self::Element], _delta: Vector3<i32>) -> Action {
        Action::NewSolids(elements.to_owned())
    }

    fn parent_tool() -> Box<dyn Tool> {
        Box::new(Hub::default())
    }

    fn element_kind() -> ElementKind {
        ElementKind::Solid
    }

    fn regen(renderer: &Renderer, elements: &[Self::Element], graphics: &mut Option<Graphics>) {
        graphics::generate(
            MeshGenInput {
                renderer,
                mask: ElementKind::Solid,
                solids: elements.iter(),
            },
            graphics,
        );
    }
}
