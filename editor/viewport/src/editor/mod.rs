mod camera;
mod scene;

use std::{default, rc::Rc};

use anyhow::Result;
use asset_id::{GizmoID, TextureID};
use cgmath::{vec3, MetricSpace, Vector2, Vector3, Zero};
use renderer::{
    data::gizmo,
    scene::{GizmoObject, LineObject, Scene as RenderScene, SolidObject},
    Renderer,
};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    editor::scene::{Action, GraphicsMask, RaycastEndpointKind, Solid},
    input::Input,
    math::{MinMax, Snap},
};

use self::{
    camera::Camera,
    scene::{Scene, WorkInProgress},
};

#[derive(Default)]
pub struct Editor {
    camera: Camera,
    scene: Scene,
    mode: Mode,
    graphics: Option<Graphics>,
}

impl Editor {
    pub fn process(&mut self, ctx: OuterContext) -> Result<()> {
        let mut needs_regen = false;
        let mut can_move = true;

        match &mut self.mode {
            Mode::Solid(state) => {
                if ctx.input.is_button_down_once(MouseButton::Left) {
                    state.last_click_pos = Some(ctx.input.mouse_pos());
                }

                if let Some(last_click_pos) = state.last_click_pos {
                    can_move = false;

                    if ctx.input.mouse_pos().distance2(last_click_pos) > 100.0 {
                        let hit = self.scene.raycast(&self.camera.screen_ray(last_click_pos));

                        state.new_solid_start =
                            Some(hit.endpoint.point + hit.endpoint.normal * 0.001);

                        *self.scene.wip() = Some(WorkInProgress::NewSolid(Solid::new(
                            Vector3::zero(),
                            Vector3::zero(),
                        )));

                        state.last_click_pos = None;
                    }
                }

                if let Some(start) = state.new_solid_start {
                    can_move = false;

                    let hit = self
                        .scene
                        .raycast(&self.camera.screen_ray(ctx.input.mouse_pos()));

                    let end = hit.endpoint.point + hit.endpoint.normal * 0.001;

                    let start = (start * 100.0).snap(100) * 100;
                    let end = (end * 100.0).snap(100) * 100;

                    let min = start.min(end);
                    let max = start.max(end) + vec3(100, 100, 100);

                    if let Some(WorkInProgress::NewSolid(solid)) = self.scene.wip() {
                        if solid.set_min_max(min, max) {
                            needs_regen = true;
                        }
                    }
                }

                if ctx.input.was_button_down_once(MouseButton::Left) {
                    state.last_click_pos = None;

                    if self.scene.wip().is_some() {
                        self.scene.confirm_wip();
                        state.new_solid_start = None;
                        needs_regen = true;
                    } else {
                        if !ctx.input.is_key_down(VirtualKeyCode::LShift) {
                            self.scene.act(Action::DeselectSolids);
                            needs_regen = true;
                        }

                        let hit = self
                            .scene
                            .raycast(&self.camera.screen_ray(ctx.input.mouse_pos()));

                        if let RaycastEndpointKind::Face { solid_id, .. } = hit.endpoint.kind {
                            self.scene.act(Action::SelectSolids(vec![solid_id]));
                            needs_regen = true;
                        }
                    }
                }

                if ctx.input.is_key_down_once(VirtualKeyCode::Delete) {
                    self.scene.act(Action::RemoveSelectedSolids);
                    needs_regen = true;
                }
            }
            Mode::Face => todo!(),
            Mode::Point => todo!(),
            Mode::Prop => todo!(),
        }

        self.undo_redo(ctx.input, ctx.renderer);

        if can_move {
            self.control_camera(ctx.input, ctx.delta);
        }

        if needs_regen {
            self.regen(ctx.renderer);
        }

        Ok(())
    }

    pub fn render(&self, renderer: &Renderer) -> Result<()> {
        let mut scene = RenderScene::default();
        scene.set_camera_matrices(self.camera.matrix(), self.camera.projection());

        if let Some(graphics) = &self.graphics {
            for mesh_object in &graphics.solid_objects {
                scene.push_solid_object(mesh_object.clone());
            }

            scene.push_line_object(graphics.line_object.clone());
            scene.push_gizmo_object(GizmoObject {
                id: GizmoID(0),
                instances: graphics.point_gizmo_instances.clone(),
            });
        }

        renderer.render(&scene)?;
        Ok(())
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.camera.recreate_projection(width, height);
    }

    fn control_camera(&mut self, input: &Input, delta: f32) {
        if !input.is_button_down(MouseButton::Right) {
            return;
        }

        if input.is_key_down(VirtualKeyCode::W) {
            self.camera.move_forward(delta);
        }

        if input.is_key_down(VirtualKeyCode::S) {
            self.camera.move_backward(delta);
        }

        if input.is_key_down(VirtualKeyCode::A) {
            self.camera.move_left(delta);
        }

        if input.is_key_down(VirtualKeyCode::D) {
            self.camera.move_right(delta);
        }

        if input.is_key_down(VirtualKeyCode::Q) {
            self.camera.move_down(delta);
        }

        if input.is_key_down(VirtualKeyCode::E) {
            self.camera.move_up(delta);
        }

        if input.mouse_wheel().abs() > 0.1 {
            if input.mouse_wheel().signum() > 0.0 {
                self.camera.increase_speed();
            } else {
                self.camera.decrease_speed();
            }
        }

        self.camera.look(input.mouse_delta(), delta);
    }

    fn undo_redo(&mut self, input: &Input, renderer: &Renderer) {
        if input.is_key_down(VirtualKeyCode::LControl) {
            if input.is_key_down_once(VirtualKeyCode::Z) {
                self.scene.undo();
                self.regen(renderer);
            } else if input.is_key_down_once(VirtualKeyCode::Y) {
                self.scene.redo();
                self.regen(renderer);
            }
        }
    }

    fn regen(&mut self, renderer: &Renderer) {
        self.scene
            .gen_graphics(renderer, &mut self.graphics, self.mode.graphics_mask());
    }
}

pub struct OuterContext<'a> {
    pub delta: f32,
    pub input: &'a Input,
    pub renderer: &'a Renderer,
}

struct Graphics {
    solid_objects: Vec<SolidObject>,
    line_object: LineObject,
    point_gizmo_instances: Rc<gizmo::Instances>,
}

enum Mode {
    Solid(SolidState),
    Face,
    Point,
    Prop,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Solid(SolidState::default())
    }
}

impl Mode {
    fn graphics_mask(&self) -> GraphicsMask {
        match self {
            Mode::Solid(_) => GraphicsMask::Solids,
            Mode::Face => GraphicsMask::Faces,
            Mode::Point => GraphicsMask::Points,
            Mode::Prop => todo!(),
        }
    }
}

#[derive(Default)]
struct SolidState {
    last_click_pos: Option<Vector2<f32>>,
    new_solid_start: Option<Vector3<f32>>,
}
