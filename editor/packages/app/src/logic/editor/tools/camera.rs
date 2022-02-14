use winit::event::{MouseButton, VirtualKeyCode};

use crate::logic::{
    elements::{ElementKind, RaycastEndpoint, RaycastEndpointKind},
    scene::{self, Action},
};

use super::{Context, Tool};

pub struct CameraTool;

impl Tool for CameraTool {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        let mut ctx = ctx;
        if ctx.input.is_button_down(MouseButton::Right) {
            control(&mut ctx);
            None
        } else {
            manipulate(&mut ctx)
        }
    }
}

fn control(ctx: &mut Context) {
    if ctx.input.is_key_down(VirtualKeyCode::W) {
        ctx.camera.move_forward(ctx.delta);
    }

    if ctx.input.is_key_down(VirtualKeyCode::S) {
        ctx.camera.move_backward(ctx.delta);
    }

    if ctx.input.is_key_down(VirtualKeyCode::A) {
        ctx.camera.move_left(ctx.delta);
    }

    if ctx.input.is_key_down(VirtualKeyCode::D) {
        ctx.camera.move_right(ctx.delta);
    }

    if ctx.input.is_key_down(VirtualKeyCode::Q) {
        ctx.camera.move_down(ctx.delta);
    }

    if ctx.input.is_key_down(VirtualKeyCode::E) {
        ctx.camera.move_up(ctx.delta);
    }

    if ctx.input.mouse_wheel().abs() > 0.1 {
        if ctx.input.mouse_wheel() > 0.0 {
            ctx.camera.increase_speed();
        } else {
            ctx.camera.decrease_speed();
        }
    }

    ctx.camera.look(ctx.input.mouse_delta(), ctx.delta);
}

fn manipulate(ctx: &mut Context) -> Option<Box<dyn Tool>> {
    // Undo / Redo
    if ctx.input.is_key_down(VirtualKeyCode::LControl) {
        if ctx.input.is_key_down_once(VirtualKeyCode::Z) {
            ctx.scene.undo(scene::Context {
                graphics: ctx.graphics,
            });
        } else if ctx.input.is_key_down_once(VirtualKeyCode::Y) {
            ctx.scene.redo(scene::Context {
                graphics: ctx.graphics,
            });
        }
    }

    // Select
    if ctx.input.was_button_down_once(MouseButton::Left) {
        if !ctx.input.is_key_down(VirtualKeyCode::LShift) {
            ctx.scene.act(
                scene::Context {
                    graphics: ctx.graphics,
                },
                Action::DeselectAll(ctx.mode),
            );
        }

        let hit = ctx.scene.raycast(ctx.input.mouse_pos(), ctx.camera);
        match ctx.mode {
            ElementKind::Solid => {
                if let Some(RaycastEndpoint {
                    kind: RaycastEndpointKind::Face(locator),
                    ..
                }) = hit.endpoint
                {
                    ctx.scene.act(
                        scene::Context {
                            graphics: ctx.graphics,
                        },
                        Action::SelectSolids(vec![locator.solid]),
                    );
                }
            }
            ElementKind::Face => {
                if let Some(RaycastEndpoint {
                    kind: RaycastEndpointKind::Face(locator),
                    ..
                }) = hit.endpoint
                {
                    ctx.scene.act(
                        scene::Context {
                            graphics: ctx.graphics,
                        },
                        Action::SelectFaces(vec![locator]),
                    );
                }
            }
            ElementKind::Point => todo!(),
            ElementKind::Prop => todo!(),
        }
    }

    None
}
