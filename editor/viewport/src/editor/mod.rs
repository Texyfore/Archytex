mod camera;
mod scene;

use std::rc::Rc;

use anyhow::Result;
use asset_id::GizmoID;
use cgmath::{vec3, Vector3, Zero};
use renderer::{
    data::gizmo,
    scene::{GizmoObject, LineObject, Scene as RenderScene, SolidObject},
    Renderer,
};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    editor::scene::{Action, GraphicsMask, Solid},
    input::Input,
};

use self::{camera::Camera, scene::Scene};

#[derive(Default)]
pub struct Editor {
    camera: Camera,
    scene: Scene,
    graphics: Option<Graphics>,
}

impl Editor {
    pub fn process(&mut self, ctx: OuterContext) -> Result<()> {
        self.control_camera(ctx.input, ctx.delta);
        self.undo_redo(ctx.input, ctx.renderer);

        if ctx.input.is_button_down_once(MouseButton::Left) {
            let hit = self
                .scene
                .raycast(&self.camera.screen_ray(ctx.input.mouse_pos()));

            if ctx.input.is_key_down(VirtualKeyCode::LControl) {
                self.scene.act(Action::AddSolid(Solid::new(
                    hit.endpoint.point.map(|e| (e * 100.0) as i32),
                    vec3(100, 100, 100),
                )));
            } else {
                self.scene.act(Action::SelectPoints(hit.points));
            }

            self.scene
                .gen_meshes(ctx.renderer, &mut self.graphics, GraphicsMask::Points);
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

        if input.is_button_down(MouseButton::Right) {
            self.camera.look(input.mouse_delta(), delta);
        }

        if input.mouse_wheel().abs() > 0.1 {
            if input.mouse_wheel().signum() > 0.0 {
                self.camera.increase_speed();
            } else {
                self.camera.decrease_speed();
            }
        }
    }

    fn undo_redo(&mut self, input: &Input, renderer: &Renderer) {
        if input.is_key_down(VirtualKeyCode::LControl) {
            if input.is_key_down_once(VirtualKeyCode::Z) {
                self.scene.undo();
                self.scene
                    .gen_meshes(renderer, &mut self.graphics, GraphicsMask::Points)
            } else if input.is_key_down_once(VirtualKeyCode::Y) {
                self.scene.redo();
                self.scene
                    .gen_meshes(renderer, &mut self.graphics, GraphicsMask::Points)
            }
        }
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
