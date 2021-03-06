use cgmath::{vec3, Vector2, Vector3};
use winit::event::MouseButton;

use crate::{
    graphics::Canvas,
    logic::{
        elements::{ElementKind, Movable, Solid},
        scene::{self, Action},
    },
    math::{MinMax, Snap},
};

use super::{CameraTool, Context, Tool};

pub struct NewSolid {
    start: Vector3<f32>,
    solid: Option<Solid>,
}

impl NewSolid {
    pub fn new(ctx: &mut Context, click: Vector2<f32>) -> Option<Self> {
        let hit = ctx.scene.raycast(click, ctx.camera, ctx.prop_infos);
        hit.endpoint.map(|endpoint| Self {
            start: endpoint.point + endpoint.normal * 0.001,
            solid: None,
        })
    }
}

impl Tool for NewSolid {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        let hit = ctx
            .scene
            .raycast(ctx.input.mouse_pos(), ctx.camera, ctx.prop_infos);
        if let Some(endpoint) = hit.endpoint {
            let g = *ctx.grid;
            let scaled_normal = endpoint.normal * g as f32 / 128.0 * 0.1;

            let start = self.start.snap(g);
            let end = (endpoint.point + scaled_normal).snap(g);
            let min = start.min(end);
            let max = start.max(end) + vec3(g, g, g);
            self.solid = Some(Solid::new(ctx.graphics, min, max - min));
        }

        if ctx.input.was_button_down_once(MouseButton::Left) {
            if let Some(solid) = self.solid.take() {
                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::NewSolids(vec![solid]),
                );
            }
            Some(Box::new(CameraTool::new(ctx.graphics)))
        } else {
            None
        }
    }

    fn render(&self, canvas: &mut Canvas) {
        if let Some(solid) = &self.solid {
            solid.render(canvas, ElementKind::Solid);
        }
    }
}
