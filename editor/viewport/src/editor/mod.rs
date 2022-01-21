mod camera;
mod scene;

use std::rc::Rc;

use anyhow::Result;
use asset_id::GizmoID;
use renderer::{
    data::gizmo,
    scene::{GizmoObject, LineObject, Scene as RenderScene, SolidObject},
    Renderer,
};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::input::Input;

use self::{camera::Camera, scene::Scene};

#[derive(Default)]
pub struct Editor {
    camera: Camera,
    scene: Scene,
    graphics: Option<Graphics>,
}

impl Editor {
    pub fn process(&mut self, ctx: OuterContext) -> Result<()> {
        if ctx.input.is_key_down(VirtualKeyCode::W) {
            self.camera.move_forward(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::S) {
            self.camera.move_backward(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::A) {
            self.camera.move_left(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::D) {
            self.camera.move_right(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::Q) {
            self.camera.move_down(ctx.delta);
        }

        if ctx.input.is_key_down(VirtualKeyCode::E) {
            self.camera.move_up(ctx.delta);
        }

        if ctx.input.is_button_down(MouseButton::Right) {
            self.camera.look(ctx.input.mouse_delta(), ctx.delta);
        }

        if ctx.input.mouse_wheel().abs() > 0.1 {
            if ctx.input.mouse_wheel().signum() > 0.0 {
                self.camera.increase_speed();
            } else {
                self.camera.decrease_speed();
            }
        }

        if ctx.input.is_key_down(VirtualKeyCode::LControl) {
            if ctx.input.is_key_down_once(VirtualKeyCode::Z) {
                self.scene.undo();
                self.regen_meshes(ctx.renderer)?;
            } else if ctx.input.is_key_down_once(VirtualKeyCode::Y) {
                self.scene.redo();
                self.regen_meshes(ctx.renderer)?;
            }
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

    fn regen_meshes(&mut self, renderer: &Renderer) -> Result<()> {
        self.scene.gen_meshes(renderer, &mut self.graphics);
        Ok(())
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
