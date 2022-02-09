mod camera;
mod input;

use asset::TextureID;
use cgmath::{vec2, vec3};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    graphics::{
        structures::{LineVertex, SolidVertex},
        Canvas, Graphics, LineMesh, LineMeshDescriptor, Share, SolidMesh, SolidMeshDescriptor,
    },
    OnSave,
};

use self::{camera::Camera, input::Input};

pub struct Logic {
    input: Input,
    camera: Camera,
    lines: LineMesh,
    solid: SolidMesh,
}

impl Logic {
    pub fn init(ctx: Context) -> Self {
        Self {
            input: Input::default(),
            camera: Camera::default(),
            lines: ctx.graphics.create_line_mesh(LineMeshDescriptor {
                vertices: &[
                    LineVertex {
                        position: vec3(0.0, 0.0, 0.0),
                        color: [1.0, 0.0, 0.0],
                    },
                    LineVertex {
                        position: vec3(5.0, 5.0, 0.0),
                        color: [0.0, 0.0, 1.0],
                    },
                ],
            }),
            solid: ctx.graphics.create_solid_mesh(SolidMeshDescriptor {
                texture: TextureID(0),
                vertices: &[
                    SolidVertex {
                        position: vec3(0.0, 0.0, 0.0),
                        normal: vec3(0.0, 0.0, 1.0),
                        texcoord: vec2(0.0, 0.0),
                        tint: [0.0; 4],
                    },
                    SolidVertex {
                        position: vec3(1.0, 0.0, 0.0),
                        normal: vec3(0.0, 0.0, 1.0),
                        texcoord: vec2(1.0, 0.0),
                        tint: [0.0; 4],
                    },
                    SolidVertex {
                        position: vec3(1.0, 1.0, 0.0),
                        normal: vec3(0.0, 0.0, 1.0),
                        texcoord: vec2(1.0, 1.0),
                        tint: [0.0; 4],
                    },
                ],
                triangles: &[[0, 1, 2]],
            }),
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
        canvas.draw_lines(self.lines.share());
        canvas.draw_solid(self.solid.share());
    }
}

pub struct Context<'a> {
    pub graphics: &'a Graphics,
    pub save_handler: &'a dyn OnSave,
    pub delta: f32,
}
