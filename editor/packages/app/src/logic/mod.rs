mod camera;
mod editor;
mod elements;
mod input;
mod scene;

use asset::PropID;
use cgmath::{vec2, Vector3, Zero};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    data::PropInfoContainer,
    graphics::{Canvas, Graphics},
    Host,
};

use self::{
    camera::Camera,
    editor::Editor,
    elements::Prop,
    input::Input,
    scene::{Action, Scene},
};

pub struct Logic {
    input: Input,
    camera: Camera,
    scene: Scene,
    editor: Editor,
}

impl Logic {
    pub fn init(ctx: Context) -> Self {
        let input = Input::default();
        let mut camera = Camera::default();
        let mut scene = Scene::default();
        scene.act(
            scene::Context {
                graphics: ctx.graphics,
            },
            Action::NewProps(vec![Prop::new(
                ctx.graphics,
                PropID(0),
                Vector3::zero(),
                Vector3::zero(),
            )]),
        );

        let editor = Editor::init(editor::Context {
            input: &input,
            graphics: ctx.graphics,
            prop_infos: ctx.prop_infos,
            camera: &mut camera,
            scene: &mut scene,
            delta: ctx.delta,
        });

        Self {
            input,
            camera,
            scene,
            editor,
        }
    }

    pub fn process(&mut self, ctx: Context) {
        self.editor.process(editor::Context {
            input: &self.input,
            graphics: ctx.graphics,
            prop_infos: ctx.prop_infos,
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
    }
}

pub struct Context<'a> {
    pub host: &'a dyn Host,
    pub graphics: &'a Graphics,
    pub prop_infos: &'a PropInfoContainer,
    pub delta: f32,
}
