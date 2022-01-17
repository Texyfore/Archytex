mod camera;
mod scene;

use anyhow::Result;
use renderer::{scene::Scene as RenderScene, Renderer};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::input::Input;

use self::{camera::Camera, scene::Scene};

#[derive(Default)]
pub struct Editor {
    camera: Camera,
    scene: Scene,
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

        Ok(())
    }

    pub fn render(&self, renderer: &Renderer) -> Result<()> {
        let mut scene = RenderScene::default();
        scene.set_camera_matrix(self.camera.matrix());
        renderer.render(&scene)?;

        Ok(())
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.camera.recreate_projection(width, height);
    }
}

pub struct OuterContext<'a> {
    pub delta: f32,
    pub input: &'a Input,
}
