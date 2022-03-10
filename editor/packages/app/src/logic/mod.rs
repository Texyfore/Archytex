mod camera;
mod common;
mod editor;
mod elements;
mod input;
mod scene;

use asset::{GizmoID, PropID, TextureID};
use cgmath::{vec2, Matrix4, Zero};
use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use crate::{
    data::PropInfoContainer,
    graphics::{structures::GizmoInstance, Canvas, GizmoGroup, GizmoInstances, Graphics, Share},
    Host, ToHost,
};

use self::{camera::Camera, editor::Editor, input::Input, scene::Scene};

pub use elements::ElementKind;

pub struct Logic {
    input: Input,
    camera: Camera,
    scene: Scene,
    editor: Editor,
    dummy: GizmoInstances,
}

impl Logic {
    pub fn init(ctx: Context) -> Self {
        let input = Input::default();
        let mut camera = Camera::default();
        let mut scene = Scene::default();

        let editor = Editor::init(editor::Context {
            host: ctx.host,
            input: &input,
            graphics: ctx.graphics,
            prop_infos: ctx.prop_infos,
            camera: &mut camera,
            scene: &mut scene,
            delta: ctx.delta,
        });

        // HACK: Drawing an invisible gizmo at all times prevents the weird whiteout bug on web
        let dummy = ctx.graphics.create_gizmo_instances(1);
        ctx.graphics.write_gizmo_instances(
            &dummy,
            &[GizmoInstance {
                matrix: Matrix4::zero(),
                color: [0.0; 3],
            }],
        );

        Self {
            input,
            camera,
            scene,
            editor,
            dummy,
        }
    }

    pub fn process(&mut self, ctx: Context) {
        self.editor.process(editor::Context {
            host: ctx.host,
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

    pub fn movement_override(&mut self, x: f32, y: f32) {
        self.input.movement_override(vec2(x, y));
    }

    pub fn scroll(&mut self, delta: f32) {
        self.input.scroll(delta);
    }

    pub fn save_scene(&self, ctx: Context, id: i32) {
        let scene = asset::scene::Scene {
            camera: self.camera.save(),
            world: self.scene.save(),
        };

        let buf = scene.encode().unwrap();
        ctx.host.callback(ToHost::SceneSaved(id, buf));
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

    pub fn set_editor_mode(&mut self, ctx: Context, mode: ElementKind) {
        self.editor.set_mode(
            editor::Context {
                host: ctx.host,
                input: &self.input,
                graphics: ctx.graphics,
                prop_infos: ctx.prop_infos,
                camera: &mut self.camera,
                scene: &mut self.scene,
                delta: ctx.delta,
            },
            mode,
        );
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.set_camera_matrices(self.camera.matrices());
        self.scene.render(canvas, self.editor.mode());
        self.editor.render(canvas);
        canvas.draw_gizmos(GizmoGroup {
            gizmo: GizmoID(0),
            instances: self.dummy.share(),
        })
    }
}

pub struct Context<'a> {
    pub host: &'a dyn Host,
    pub graphics: &'a Graphics,
    pub prop_infos: &'a PropInfoContainer,
    pub delta: f32,
}
