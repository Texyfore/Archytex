use asset_id::{GizmoID, TextureID};
use cgmath::Vector2;
use instant::Instant;
use renderer::{data::gizmo, scene::Scene, Renderer};
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::{
    editor::{self, Editor},
    input::Input,
    ipc::IpcHost,
};

pub struct MainLoop {
    renderer: Renderer,
    input: Input,
    editor: Editor,
    before: Instant,
}

impl MainLoop {
    pub fn new(window: &Window) -> Self {
        let mut renderer = Renderer::new(window).unwrap();
        let input = Input::default();
        let mut editor = Editor::default();

        renderer
            .load_texture(TextureID(0), include_bytes!("nodraw.png"))
            .unwrap();
        renderer
            .load_texture(TextureID(1), include_bytes!("bricks.png"))
            .unwrap();

        {
            use formats::agzm;
            let mesh = agzm::Mesh::decode(include_bytes!("gizmo.agzm")).unwrap();

            let vertices = mesh
                .vertices
                .into_iter()
                .map(|vertex| gizmo::Vertex {
                    position: vertex.position,
                })
                .collect::<Vec<_>>();

            renderer.load_gizmo(GizmoID(0), &vertices, &mesh.triangles);
        }

        {
            let (width, height) = window.inner_size().into();
            renderer.resize(width, height);
            editor.window_resized(width, height);
        }

        Self {
            renderer,
            input,
            editor,
            before: Instant::now(),
        }
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) {
        let after = Instant::now();
        let delta = (after - self.before).as_secs_f32();
        self.before = after;

        self.editor.process(editor::Context {
            delta,
            input: &self.input,
            renderer: &self.renderer,
        });

        self.input.process();
    }

    pub fn render(&self) {
        let mut scene = Scene::default();
        self.editor.render(&mut scene);
        self.renderer.render(&scene).unwrap();
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.editor.window_resized(width, height);
        self.renderer.resize(width, height);
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.input.keyboard_input(key, state);
    }

    pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.input.mouse_input(button, state);
    }

    pub fn mouse_movement(&mut self, new_pos: Vector2<f32>) {
        self.input.mouse_movement(new_pos);
    }

    pub fn mouse_wheel_movement(&mut self, movement: f32) {
        self.input.mouse_wheel_movement(movement);
    }
}
