use std::rc::Rc;

use renderer::{
    scene::{MeshObject, Scene},
    Renderer,
};
use thiserror::Error;
use tk3d::{
    math::{vec2, vec3},
    TextureID, Triangle, Vertex,
};
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::{input::Input, ipc::IpcHost};

pub struct MainLoop {
    renderer: Renderer,
    input: Input,
}

impl MainLoop {
    pub fn new(window: &Window) -> Result<Self, NewError> {
        let mut renderer = Renderer::new(window)?;
        renderer.load_texture(TextureID(0), include_bytes!("nodraw.png")).unwrap();

        Ok(Self {
            renderer,
            input: Input::default(),
        })
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<(), ProcessError> {
        self.input.process();
        Ok(())
    }

    pub fn render(&self) -> Result<(), RenderError> {
        let mut scene = Scene::default();
        scene.push_mesh_object(Rc::new(MeshObject {
            texture_id: TextureID(0),
            transform: self.renderer.create_transform(),
            mesh: self.renderer.create_mesh(
                &[
                    Vertex {
                        position: vec3(0.0, 0.0, -5.0),
                        normal: vec3(0.0, 0.0, 1.0),
                        texcoord: vec2(0.0, 0.0),
                    },
                    Vertex {
                        position: vec3(1.0, 0.0, -5.0),
                        normal: vec3(0.0, 0.0, 1.0),
                        texcoord: vec2(1.0, 0.0),
                    },
                    Vertex {
                        position: vec3(1.0, 1.0, -5.0),
                        normal: vec3(0.0, 0.0, 1.0),
                        texcoord: vec2(1.0, 1.0),
                    },
                ],
                &[Triangle { indices: [0, 1, 2] }],
            ),
        }));

        self.renderer.render(&mut scene)?;
        Ok(())
    }

    pub fn window_resized(&self, width: u32, height: u32) {
        self.renderer.resize(width, height);
    }

    pub fn keyboard_input(&mut self, key: VirtualKeyCode, state: ElementState) {
        self.input.keyboard_input(key, state);
    }

    pub fn mouse_input(&mut self, button: MouseButton, state: ElementState) {
        self.input.mouse_input(button, state);
    }
}

#[derive(Error, Debug)]
pub enum NewError {
    #[error("Couldn't create main loop: {0}")]
    NoRenderer(#[from] renderer::NewError),
}

#[derive(Error, Debug)]
#[error("Couldn't compute next frame")]
pub struct ProcessError;

#[derive(Error, Debug)]
#[error("{0}")]
pub struct RenderError(#[from] renderer::RenderError);
