mod camera;
mod editor;
mod elements;
mod input;
mod scene;

use asset::{PropID, TextureID};
use cgmath::vec2;
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    data::PropInfoContainer,
    graphics::{Canvas, Graphics},
    Host, ToHost,
};

use self::{camera::Camera, editor::Editor, input::Input, scene::Scene};

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

    pub fn save_scene(&self, ctx: Context) {
        let scene = asset::scene::Scene {
            camera: self.camera.save(),
            world: self.scene.save(),
        };

        let buf = scene.encode().unwrap();
        ctx.host.callback(ToHost::SceneSaved(buf));
    }

    pub fn load_scene(&mut self, ctx: Context, scene: &asset::scene::Scene) {
        self.camera.load(&scene.camera);
        self.scene.load(
            scene::Context {
                graphics: ctx.graphics,
            },
            &scene.world,
        );
    }

    pub fn set_texture(&mut self, texture: TextureID) {
        self.editor.set_texture(texture);
    }

    pub fn set_prop(&mut self, prop: PropID) {
        self.editor.set_prop(prop);
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
