mod camera;
mod editor;
mod elements;
mod input;
mod scene;

use asset::PropID;
use cgmath::{vec2, Matrix4, SquareMatrix};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    graphics::{structures::TransformTint, Canvas, Graphics, PropData, PropInstance, Share},
    Host,
};

use self::{camera::Camera, editor::Editor, input::Input, scene::Scene};

pub struct Logic {
    input: Input,
    camera: Camera,
    scene: Scene,
    editor: Editor,
    asd: PropData,
}

impl Logic {
    pub fn init(ctx: Context) -> Self {
        let input = Input::default();
        let mut camera = Camera::default();
        let mut scene = Scene::default();

        let editor = Editor::init(editor::Context {
            input: &input,
            graphics: ctx.graphics,
            camera: &mut camera,
            scene: &mut scene,
            delta: ctx.delta,
        });

        Self {
            input,
            camera,
            scene,
            editor,
            asd: ctx.graphics.create_prop_data(&TransformTint {
                transform: Matrix4::identity(),
                tint: [0.0; 4],
            }),
        }
    }

    pub fn process(&mut self, ctx: Context) {
        self.editor.process(editor::Context {
            input: &self.input,
            graphics: ctx.graphics,
            camera: &mut self.camera,
            scene: &mut self.scene,
            delta: ctx.delta,
        });
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
        self.scene.render(canvas, self.editor.mode());
        self.editor.render(canvas);
        canvas.draw_prop(PropInstance {
            prop: PropID(0),
            data: self.asd.share(),
        });
    }
}

pub struct Context<'a> {
    pub host: &'a dyn Host,
    pub graphics: &'a Graphics,
    pub delta: f32,
}
