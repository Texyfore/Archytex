use cgmath::{MetricSpace, Vector2, Vector3, Zero};
use winit::event::MouseButton;

use crate::{
    editor::scene::{
        Action, GraphicsMask, RaycastEndpointKind, RaycastHit, Scene, Solid, WorkInProgress,
    },
    math::{MinMax, Snap},
};

use super::{generic::Select as GenericSelect, Context, Tool};

#[derive(Default)]
pub struct Select {
    last_click: Option<Vector2<f32>>,
}

impl Tool for Select {
    fn process(&mut self, ctx: &mut Context) {
        if ctx.input().is_button_down_once(MouseButton::Left) {
            self.last_click = Some(ctx.input().mouse_pos());
        }

        if let Some(last_click) = self.last_click {
            if last_click.distance2(ctx.input().mouse_pos()) > 100.0 {
                let mouse_pos = ctx.input().mouse_pos();
                let ray = ctx.camera().screen_ray(mouse_pos);

                if let Some(hit) = ctx.scene().raycast(&ray) {
                    let add = Add::new(ctx.scene(), hit);
                    ctx.switch_to(add);
                }
            }
        }

        if ctx.input().was_button_down_once(MouseButton::Left) {
            self.last_click = None;
        }

        if !self.generic_process(ctx) {
            self.process_undo_redo(ctx);
            self.process_camera(ctx);
        }
    }

    fn cancelled(&mut self, _ctx: &mut Context) {}

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}

impl GenericSelect for Select {
    fn deselect_action(&self) -> Action {
        Action::DeselectSolids
    }

    fn select_action(&self, hit: RaycastHit) -> Option<Action> {
        if let RaycastEndpointKind::Face { solid_id, .. } = hit.endpoint.kind {
            Some(Action::SelectSolids(vec![solid_id]))
        } else {
            None
        }
    }
}

pub struct Add {
    start: Vector3<f32>,
}

impl Add {
    fn new(scene: &mut Scene, hit: RaycastHit) -> Self {
        *scene.wip() = WorkInProgress::NewSolid(Solid::new(Vector3::zero(), Vector3::zero()));
        Self {
            start: hit.endpoint.point + hit.endpoint.normal * 0.0001,
        }
    }
}

impl Tool for Add {
    fn process(&mut self, ctx: &mut Context) {
        let mouse_pos = ctx.input().mouse_pos();
        let ray = ctx.camera().screen_ray(mouse_pos);

        if let Some(hit) = ctx.scene().raycast(&ray) {
            let end = hit.endpoint.point + hit.endpoint.normal * 0.001;
            
            let start = self.start.snap(100);
            let end = end.snap(100);

            let min = start.min(end);
            let max = start.max(end).map(|e| e + 100);

            if ctx.scene().wip().set_min_max(min, max) {
                ctx.set_regen();
            }
        }

        if ctx.input().was_button_down_once(MouseButton::Left) {
            ctx.scene().confirm_wip();
            ctx.switch_to(Select::default());
        }
    }

    fn cancelled(&mut self, ctx: &mut Context) {
        ctx.scene().cancel_wip();
    }

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}
