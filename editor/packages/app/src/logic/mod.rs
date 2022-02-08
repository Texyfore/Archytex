mod input;

use asset::TextureID;
use cgmath::{perspective, vec2, vec3, Deg, Matrix4};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    graphics::{line, solid, Camera, Canvas, Renderer, Share},
    OnSave,
};

use self::input::Input;

pub struct Logic {
    input: Input,
    line: line::Object,
    solid: solid::Object,
}

impl Logic {
    pub fn init(ctx: Context) -> Self {
        Self {
            input: Input::default(),
            line: ctx.renderer.create_line_object(line::Mesh {
                vertices: &[
                    line::Vertex {
                        position: vec3(0.0, 0.0, 0.0),
                        color: [1.0, 0.0, 0.0],
                    },
                    line::Vertex {
                        position: vec3(1.0, 1.0, 0.0),
                        color: [0.0, 1.0, 0.0],
                    },
                ],
            }),
            solid: ctx.renderer.create_solid_object(solid::Mesh {
                texture: TextureID(0),
                vertices: &[
                    solid::Vertex {
                        position: vec3(0.0, 0.0, 0.0),
                        normal: vec3(0.0, 0.0, 1.0),
                        texcoord: vec2(0.0, 0.0),
                        tint: [0.0; 4],
                    },
                    solid::Vertex {
                        position: vec3(1.0, 0.0, 0.0),
                        normal: vec3(0.0, 0.0, 1.0),
                        texcoord: vec2(1.0, 0.0),
                        tint: [0.0; 4],
                    },
                    solid::Vertex {
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

    pub fn process(&mut self, _ctx: Context) {
        self.input.process();
    }

    pub fn resized(&mut self, _width: u32, _height: u32) {}

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
        canvas.set_camera(Camera {
            world_to_clip: perspective(Deg(80.0), 800.0 / 600.0, 0.1, 100.0)
                * Matrix4::from_translation(vec3(0.0, 0.0, -5.0)),
            view_to_world: Matrix4::from_translation(vec3(0.0, 0.0, 5.0)),
        });

        canvas.draw_lines(self.line.share());
        canvas.draw_solid(self.solid.share());
    }
}

pub struct Context<'a> {
    pub renderer: &'a Renderer,
    pub save_handler: &'a dyn OnSave,
}
