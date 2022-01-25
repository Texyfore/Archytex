use cgmath::{vec3, MetricSpace, Vector2, Vector3, Zero};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    editor::scene::{Action, GraphicsMask, RaycastEndpointKind, Solid, WorkInProgress},
    math::{MinMax, Snap},
};

use super::{OuterContext, Tool, ToolOutput};

#[derive(Default)]
pub struct Select;

impl Tool for Select {
    fn process(&mut self, ctx: &mut OuterContext) -> ToolOutput {
        let mut regen = false;
        let mut can_move = true;

        if ctx.input.is_button_down_once(MouseButton::Left) {
            if !ctx.input.is_key_down(VirtualKeyCode::LShift) {
                ctx.scene.act(Action::DeselectSolids);
                regen = true;
            }

            let hit = ctx
                .scene
                .raycast(&ctx.camera.screen_ray(ctx.input.mouse_pos()));

            if let Some(hit) = hit {
                if let RaycastEndpointKind::Face { solid_id, .. } = hit.endpoint.kind {
                    ctx.scene.act(Action::SelectSolids(vec![solid_id]));
                    regen = true;
                }

                can_move = false;
            }
        }

        if ctx.input.is_key_down_once(VirtualKeyCode::Delete) {
            ctx.scene.act(Action::RemoveSelectedSolids);
            regen = true;
        }

        if ctx.input.is_key_down_once(VirtualKeyCode::R) {
            return ToolOutput {
                switch_to: Some(Box::new(Place::default())),
                can_move,
                regen,
            };
        }

        ToolOutput {
            switch_to: None,
            can_move,
            regen,
        }
    }

    fn cancelled(&mut self, _ctx: &mut OuterContext) {}

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}

#[derive(Default)]
pub struct Place {
    last_click_pos: Option<Vector2<f32>>,
    new_solid_start: Option<Vector3<f32>>,
}

impl Tool for Place {
    fn process(&mut self, ctx: &mut OuterContext) -> ToolOutput {
        let mut can_move = true;
        let mut regen = false;

        if ctx.input.is_button_down_once(MouseButton::Left) {
            self.last_click_pos = Some(ctx.input.mouse_pos());
            println!("last click pos");
        }

        if let Some(last_click_pos) = self.last_click_pos {
            can_move = false;
            if ctx.input.mouse_pos().distance2(last_click_pos) > 100.0 {
                if let Some(hit) = ctx.scene.raycast(&ctx.camera.screen_ray(last_click_pos)) {
                    self.new_solid_start = Some(hit.endpoint.point + hit.endpoint.normal * 0.001);

                    *ctx.scene.wip() =
                        WorkInProgress::NewSolid(Solid::new(Vector3::zero(), Vector3::zero()));

                    self.last_click_pos = None;
                }
            }
        }

        if let Some(start) = self.new_solid_start {
            can_move = false;

            if let Some(hit) = ctx
                .scene
                .raycast(&ctx.camera.screen_ray(ctx.input.mouse_pos()))
            {
                let end = hit.endpoint.point + hit.endpoint.normal * 0.001;

                let start = (start * 100.0).snap(100) * 100;
                let end = (end * 100.0).snap(100) * 100;

                let min = start.min(end);
                let max = start.max(end) + vec3(100, 100, 100);

                if ctx.scene.wip().set_min_max(min, max) {
                    regen = true;
                }
            }
        }

        if ctx.input.was_button_down_once(MouseButton::Left) {
            self.last_click_pos = None;
            ctx.scene.confirm_wip();
            self.new_solid_start = None;
            regen = true;
        }

        if ctx.input.is_key_down_once(VirtualKeyCode::Escape) {
            return ToolOutput {
                switch_to: Some(Box::new(Select::default())),
                can_move,
                regen,
            };
        }

        ToolOutput {
            switch_to: None,
            can_move,
            regen,
        }
    }

    fn cancelled(&mut self, ctx: &mut OuterContext) {
        ctx.scene.cancel_wip();
    }

    fn graphics_mask(&self) -> GraphicsMask {
        GraphicsMask::Solids
    }
}
