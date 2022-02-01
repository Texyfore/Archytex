use cgmath::Vector2;
use instant::Instant;
use renderer::{scene::Scene, Renderer};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::{
    editor::{self, Editor},
    input::Input,
    ipc::{EditorMode, IpcHost, IpcMessage},
};

pub struct MainLoop {
    window: Window,
    renderer: Renderer,
    input: Input,
    editor: Editor,
    before: Instant,
}

impl MainLoop {
    pub fn new(window: Window) -> Self {
        let mut renderer = Renderer::new(&window).unwrap();
        let input = Input::default();
        let mut editor = Editor::default();

        {
            let (width, height) = window.inner_size().into();
            renderer.resize(width, height);
            editor.window_resized(width, height);
        }

        Self {
            window,
            renderer,
            input,
            editor,
            before: Instant::now(),
        }
    }

    pub fn process<H: IpcHost>(&mut self, host: &H) {
        let after = Instant::now();
        let delta = (after - self.before).as_secs_f32();
        self.before = after;

        while let Some(message) = host.recv() {
            match message {
                IpcMessage::Resources {
                    textures, gizmos, ..
                } => {
                    for (id, buf) in textures {
                        self.renderer.load_texture(id, &buf).unwrap();
                    }

                    for (id, buf) in gizmos {
                        self.renderer.load_gizmo(id, &buf).unwrap();
                    }
                }
                IpcMessage::Resolution { width, height } => {
                    self.window.set_inner_size(PhysicalSize::new(width, height));
                }
                IpcMessage::EditorMode(mode) => {
                    match mode {
                        EditorMode::Solid => self.editor.change_tool(0),
                        EditorMode::Face => self.editor.change_tool(1),
                        EditorMode::Point => self.editor.change_tool(2),
                        EditorMode::Prop => self.editor.change_tool(3),
                    };
                }
                IpcMessage::GridStep(step) => self.editor.set_grid_step(step),
                IpcMessage::CameraSpeed(speed) => self.editor.set_camera_speed(speed),
                IpcMessage::CurrentTexture(texture) => self.editor.set_current_texture(texture),
                IpcMessage::CurrentProp(prop) => self.editor.set_current_prop(prop),
                IpcMessage::RequestCameraSpeed => {
                    host.send_camera_speed(self.editor.request_camera_speed());
                }
                IpcMessage::RequestGridStep => {
                    host.send_grid_step(self.editor.request_grid_step());
                }
                IpcMessage::RequestSceneDump => {
                    host.send_scene_dump(&self.editor.request_scene_dump());
                }
            }
        }

        self.editor.process(editor::Context {
            delta,
            host,
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
