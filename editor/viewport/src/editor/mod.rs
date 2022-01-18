mod camera;
mod scene;

use anyhow::Result;
use cgmath::vec3;
use renderer::{
    scene::{MeshObject, Scene as RenderScene},
    Renderer,
};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::input::Input;

use self::{
    camera::Camera,
    scene::{Action, Scene, Solid},
};

#[derive(Default)]
pub struct Editor {
    camera: Camera,
    scene: Scene,
    mesh_cache: Vec<MeshObject>,
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

        if ctx.input.is_button_down_once(MouseButton::Left) {
            self.scene.act(Action::AddSolid(Solid::new(
                vec3(0.0, 0.0, 0.0),
                vec3(4.0, 4.0, 4.0),
            )));
            self.regen_meshes(ctx.renderer)?;
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
        scene.set_camera_matrix(self.camera.matrix());

        for mesh_object in &self.mesh_cache {
            scene.push_mesh_object(mesh_object.clone());
        }

        renderer.render(&scene)?;
        Ok(())
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.camera.recreate_projection(width, height);
    }

    fn regen_meshes(&mut self, renderer: &Renderer) -> Result<()> {
        Ok(())
    }
}

pub struct OuterContext<'a> {
    pub delta: f32,
    pub input: &'a Input,
    pub renderer: &'a Renderer,
}
