use cgmath::{InnerSpace, Vector2};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    graphics::{Canvas, Graphics},
    logic::{
        editor::gizmo::{RotationGizmo, TranslationGizmo},
        elements::{ElementKind, Movable, Prop, RaycastEndpoint, RaycastEndpointKind, Solid},
        scene::{self, Action},
    },
    math::Snap,
};

use super::{
    gizmo_move::GizmoMove, gizmo_rotate::GizmoRotate, move_tool::MoveTool,
    rotate_solid::RotateSolid, rotate_tool::RotateTool, Context, NewSolid, Tool,
};

pub struct CameraTool {
    last_click: Option<Vector2<f32>>,
    translation_gizmo: TranslationGizmo,
    rotation_gizmo: RotationGizmo,
}

impl CameraTool {
    pub fn new(graphics: &Graphics) -> Self {
        Self {
            last_click: None,
            translation_gizmo: TranslationGizmo::new(graphics),
            rotation_gizmo: RotationGizmo::new(graphics),
        }
    }

    fn common(&mut self, ctx: &mut Context) -> Option<Box<dyn Tool>> {
        // Undo & Redo
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

        // Grid
        if ctx.input.is_key_down_once(VirtualKeyCode::O) {
            *ctx.grid = (*ctx.grid / 2).clamp(2, 512);
            println!("grid: {}cm", *ctx.grid);
        }
        if ctx.input.is_key_down_once(VirtualKeyCode::P) {
            *ctx.grid = (*ctx.grid * 2).clamp(2, 512);
            println!("grid: {}cm", *ctx.grid);
        }

        // Select
        if ctx.input.was_button_down_once(MouseButton::Left)
            && !ctx.input.is_key_down(VirtualKeyCode::LControl)
        {
            if !ctx.input.is_key_down(VirtualKeyCode::LShift) {
                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::DeselectAll(ctx.mode),
                );
            }

            let hit = ctx
                .scene
                .raycast(ctx.input.mouse_pos(), ctx.camera, ctx.prop_infos);
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
                ElementKind::Point => {
                    ctx.scene.act(
                        scene::Context {
                            graphics: ctx.graphics,
                        },
                        Action::SelectPoints(hit.points),
                    );
                }
                ElementKind::Prop => {
                    if let Some(RaycastEndpoint {
                        kind: RaycastEndpointKind::Prop(index),
                        ..
                    }) = hit.endpoint
                    {
                        ctx.scene.act(
                            scene::Context {
                                graphics: ctx.graphics,
                            },
                            Action::SelectProps(vec![index]),
                        );
                    }
                }
            }
        }

        // Select all
        if ctx.input.is_key_down_once(VirtualKeyCode::A)
            && ctx.input.is_key_down(VirtualKeyCode::LShift)
        {
            match ctx.mode {
                ElementKind::Solid | ElementKind::Prop => {
                    ctx.scene.act(
                        scene::Context {
                            graphics: ctx.graphics,
                        },
                        Action::SelectAll(ctx.mode),
                    );
                }
                _ => (),
            }
        }

        // Delete
        if ctx.input.is_key_down_once(VirtualKeyCode::Delete) {
            match ctx.mode {
                ElementKind::Solid => ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::DeleteSolids,
                ),
                ElementKind::Prop => {
                    ctx.scene.act(
                        scene::Context {
                            graphics: ctx.graphics,
                        },
                        Action::DeleteProps,
                    );
                }
                _ => (),
            }
        }

        // Move

        let init_move = ctx.input.is_key_down_once(VirtualKeyCode::G);
        let init_clone = ctx.input.is_key_down_once(VirtualKeyCode::C) && !init_move;

        if init_move || init_clone {
            match ctx.mode {
                ElementKind::Solid => {
                    let ray = ctx.camera.screen_ray(ctx.input.mouse_pos());

                    let elements = if init_clone {
                        ctx.scene.clone_solids(scene::Context {
                            graphics: ctx.graphics,
                        })
                    } else {
                        ctx.scene.take_solids(ElementKind::Solid)
                    };

                    if !elements.is_empty() {
                        match MoveTool::new(ElementKind::Solid, ray, elements, init_clone) {
                            Ok(tool) => return Some(Box::new(tool)),
                            Err(elements) => {
                                if !init_clone {
                                    Solid::insert(ctx.scene, elements);
                                }
                            }
                        };
                    }
                }
                ElementKind::Face => {
                    let ray = ctx.camera.screen_ray(ctx.input.mouse_pos());
                    let elements = ctx.scene.take_solids(ElementKind::Face);
                    if !elements.is_empty() {
                        match MoveTool::new(ElementKind::Face, ray, elements, false) {
                            Ok(tool) => return Some(Box::new(tool)),
                            Err(elements) => {
                                Solid::insert(ctx.scene, elements);
                            }
                        };
                    }
                }
                ElementKind::Point => {
                    let ray = ctx.camera.screen_ray(ctx.input.mouse_pos());
                    let elements = ctx.scene.take_solids(ElementKind::Point);
                    if !elements.is_empty() {
                        match MoveTool::new(ElementKind::Point, ray, elements, false) {
                            Ok(tool) => return Some(Box::new(tool)),
                            Err(elements) => {
                                Solid::insert(ctx.scene, elements);
                            }
                        };
                    }
                }
                ElementKind::Prop => {
                    let ray = ctx.camera.screen_ray(ctx.input.mouse_pos());

                    let elements = if init_clone {
                        ctx.scene.clone_props(scene::Context {
                            graphics: ctx.graphics,
                        })
                    } else {
                        ctx.scene.take_props()
                    };

                    if !elements.is_empty() {
                        match MoveTool::new(ElementKind::Prop, ray, elements, init_clone) {
                            Ok(tool) => return Some(Box::new(tool)),
                            Err(elements) => {
                                if !init_clone {
                                    Prop::insert(ctx.scene, elements);
                                }
                            }
                        };
                    }
                }
            }
        }

        // Gizmos

        self.rotation_gizmo.set_visible(false);
        if ctx.mode == ElementKind::Prop && !ctx.input.is_key_down(VirtualKeyCode::LShift) {
            if let Some(center) = ctx.scene.calc_center(ElementKind::Prop) {
                self.rotation_gizmo.set_position(center);
                self.rotation_gizmo.set_visible(true);

                if let Some(axis) = self
                    .rotation_gizmo
                    .process(ctx.graphics, ctx.camera, ctx.input)
                {
                    let props = ctx.scene.take_props();
                    return Some(Box::new(GizmoRotate::new(
                        ctx.graphics,
                        ctx.camera,
                        ctx.input,
                        props,
                        center,
                        axis,
                    )));
                }
            }
        }

        self.translation_gizmo.set_visible(false);
        if let Some(center) = ctx.scene.calc_center(ctx.mode) {
            if !ctx.input.is_key_down(VirtualKeyCode::LShift) {
                self.translation_gizmo.set_position(center);
                self.translation_gizmo.set_visible(true);

                if let Some(selection) =
                    self.translation_gizmo
                        .process(ctx.graphics, ctx.camera, ctx.input)
                {
                    let ray = ctx.camera.screen_ray(ctx.input.mouse_pos());

                    match ctx.mode {
                        ElementKind::Solid | ElementKind::Face | ElementKind::Point => {
                            let elements = ctx.scene.take_solids(ctx.mode);

                            let tool = GizmoMove::new(
                                ctx.graphics,
                                &ray,
                                center,
                                ctx.mode,
                                selection,
                                elements,
                            );

                            match tool {
                                Ok(tool) => return Some(Box::new(tool)),
                                Err(elements) => {
                                    ctx.scene.insert_solids(elements);
                                }
                            };
                        }
                        ElementKind::Prop => {
                            let elements = ctx.scene.take_props();

                            let tool = GizmoMove::new(
                                ctx.graphics,
                                &ray,
                                center,
                                ctx.mode,
                                selection,
                                elements,
                            );

                            match tool {
                                Ok(tool) => return Some(Box::new(tool)),
                                Err(elements) => {
                                    ctx.scene.insert_props(elements);
                                }
                            };
                        }
                    }
                }
            }
        }

        None
    }
}

impl Tool for CameraTool {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        let mut ctx = ctx;
        if ctx.input.is_button_down(MouseButton::Right) {
            control(&mut ctx);
            None
        } else {
            if matches!(ctx.mode, ElementKind::Solid) {
                if ctx.input.is_button_down_once(MouseButton::Left) {
                    self.last_click = Some(ctx.input.mouse_pos());
                }

                if ctx.input.was_button_down_once(MouseButton::Left) {
                    self.last_click = None;
                }

                // New solid
                if let Some(last_click) = self.last_click {
                    let delta = ctx.input.mouse_pos() - last_click;
                    if delta.magnitude2() > 100.0 {
                        let tool = NewSolid::new(&mut ctx, last_click);

                        if let Some(tool) = tool {
                            ctx.scene.act(
                                scene::Context {
                                    graphics: ctx.graphics,
                                },
                                Action::DeselectAll(ElementKind::Solid),
                            );
                            return Some(Box::new(tool));
                        }
                    }
                }

                // Rotate
                if ctx.input.is_key_down_once(VirtualKeyCode::F) && ctx.scene.any_solids_selected()
                {
                    return Some(Box::new(RotateSolid));
                }

                // Hollow
                if ctx.input.is_key_down_once(VirtualKeyCode::H) {
                    let ids = ctx.scene.selected_solid_ids();
                    for id in ids {
                        ctx.scene.act(
                            scene::Context {
                                graphics: ctx.graphics,
                            },
                            Action::ReplaceSolids {
                                ids: vec![id],
                                solids: ctx.scene.hollow_of(ctx.graphics, id, *ctx.grid),
                            },
                        )
                    }
                }
            } else {
                self.last_click = None;
            }

            // Assign texture
            if matches!(ctx.mode, ElementKind::Face)
                && ctx.input.is_key_down_once(VirtualKeyCode::T)
            {
                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::AssignTexture(ctx.texture),
                );
            }

            if let Some(ret) = self.common(&mut ctx) {
                return Some(ret);
            }

            if matches!(ctx.mode, ElementKind::Prop) {
                // New prop
                if ctx.input.is_key_down(VirtualKeyCode::LControl)
                    && ctx.input.is_button_down_once(MouseButton::Left)
                {
                    let hit = ctx
                        .scene
                        .raycast(ctx.input.mouse_pos(), ctx.camera, ctx.prop_infos);

                    if let Some(endpoint) = hit.endpoint {
                        let position = (endpoint.point + endpoint.normal * 0.001).round(*ctx.grid);

                        ctx.scene.act(
                            scene::Context {
                                graphics: ctx.graphics,
                            },
                            Action::NewProps(vec![Prop::new(ctx.graphics, ctx.prop, position)]),
                        );
                    }
                }

                // Rotate
                if ctx.input.is_key_down_once(VirtualKeyCode::R) {
                    let props = ctx.scene.take_props();
                    if !props.is_empty() {
                        match RotateTool::new(&ctx, props) {
                            Ok(tool) => {
                                return Some(Box::new(tool));
                            }
                            Err(props) => ctx.scene.insert_props(props),
                        }
                    }
                }
            }

            None
        }
    }

    fn render(&self, canvas: &mut Canvas) {
        self.translation_gizmo.render(canvas);
        self.rotation_gizmo.render(canvas);
    }

    fn can_switch(&self) -> bool {
        true
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
