mod camera;
mod input;

use asset::GizmoID;
use cgmath::{vec2, vec3, Matrix4};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    graphics::{structures::GizmoInstance, Canvas, GizmoGroup, GizmoInstances, Graphics, Share},
    OnSave,
};

use self::{camera::Camera, input::Input};

pub struct Logic {
    input: Input,
    camera: Camera,
    instances: GizmoInstances,
}

impl Logic {
    pub fn init(ctx: Context) -> Self {
        Self {
            input: Input::default(),
            camera: Camera::default(),
            instances: ctx.graphics.create_gizmo_instances(&[
                GizmoInstance {
                    matrix: Matrix4::from_translation(vec3(0.0, 0.0, 0.0)),
                    color: [1.0, 0.0, 0.0],
                },
                GizmoInstance {
                    matrix: Matrix4::from_translation(vec3(0.0, 2.0, 0.0)),
                    color: [0.0, 1.0, 0.0],
                },
                GizmoInstance {
                    matrix: Matrix4::from_translation(vec3(0.0, 4.0, 0.0)),
                    color: [0.0, 0.0, 1.0],
                },
            ]),
        }
    }

    pub fn process(&mut self, ctx: Context) {
        if self.input.is_key_down(VirtualKeyCode::W) {
            self.camera.move_forward(ctx.delta);
        }
        if self.input.is_key_down(VirtualKeyCode::S) {
            self.camera.move_backward(ctx.delta);
        }
        self.input.process();
    }

    pub fn resized(&mut self, width: u32, height: u32) {
        self.camera.recalc(width, height);
    }

    pub fn key(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.input.key(key, state);
    }

    pub fn button(&mut self, button: MouseButton, state: ElementState) {
        self.input.button(button, state);
    }

    pub fn movement(&mut self, x: f32, y: f32) {
        self.input.movement(vec2(x, y));
    }

    pub fn scroll(&mut self, delta: f32) {
        self.input.scroll(delta);
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.set_camera_matrices(self.camera.matrices());
        canvas.draw_gizmos(GizmoGroup {
            gizmo: GizmoID(0),
            instances: self.instances.share(),
        });
    }
}

pub struct Context<'a> {
    pub graphics: &'a Graphics,
    pub save_handler: &'a dyn OnSave,
    pub delta: f32,
}
