mod camera;
mod graphics;
mod scene;
mod tools;

use std::marker::PhantomData;

use asset_id::GizmoID;
use renderer::{
    scene::{GizmoObject, Scene as RenderScene},
    Renderer,
};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::input::Input;

use self::{
    camera::Camera,
    graphics::{mesh_gen, Graphics, MeshGenInput},
    scene::Scene,
    tools::{solid, Tool},
};

pub struct Editor {
    camera: Camera,
    scene: Scene,
    tool: Box<dyn Tool>,
    graphics: Option<Graphics>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            scene: Scene::default(),
            tool: Box::new(solid::Select::default()),
            graphics: None,
        }
    }
}

impl Editor {
    pub fn process(&mut self, ctx: Context) {
        let mut tool_ctx =
            tools::Context::new(ctx.delta, ctx.input, &mut self.camera, &mut self.scene);

        self.tool.process(&mut tool_ctx);

        if let Some(next_tool) = tool_ctx.take_next_tool() {
            self.tool.cancelled(&mut tool_ctx);
            self.tool = next_tool;
        }

        if tool_ctx.regen() {
            mesh_gen(
                MeshGenInput {
                    renderer: ctx.renderer,
                    mask: self.tool.graphics_mask(),
                    solids: self.scene.iter_solids(),
                    _f: PhantomData,
                    _p: PhantomData,
                },
                &mut self.graphics,
            );
        }
    }

    pub fn render(&self, scene: &mut RenderScene) {
        scene.set_camera_matrices(self.camera.matrix(), self.camera.projection());

        if let Some(graphics) = &self.graphics {
            for mesh_object in &graphics.solid_objects {
                scene.push_solid_object(mesh_object.clone());
            }

            scene.push_line_object(graphics.line_object.clone());
            scene.push_gizmo_object(GizmoObject {
                id: GizmoID(0),
                instances: graphics.point_gizmos.clone(),
            });
        }
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

    fn undo_redo(&mut self, input: &Input) -> bool {
        if input.is_key_down(VirtualKeyCode::LControl) {
            if input.is_key_down_once(VirtualKeyCode::Z) {
                self.scene.undo();
                return true;
            } else if input.is_key_down_once(VirtualKeyCode::Y) {
                self.scene.redo();
                return true;
            }
        }

        false
    }
}

pub struct Context<'a> {
    pub delta: f32,
    pub input: &'a Input,
    pub renderer: &'a Renderer,
}
