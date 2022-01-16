use std::rc::Rc;

use renderer::{
    data::GizmoInstance,
    scene::{GizmoObject, Scene},
    Renderer,
};
use thiserror::Error;
use tk3d::math::{vec3, Matrix4};
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::{input::Input, ipc::IpcHost};

pub struct MainLoop {
    renderer: Renderer,
    input: Input,
    gizmo_object: Rc<GizmoObject>,
}

impl MainLoop {
    pub fn new(window: &Window) -> Result<Self, NewError> {
        let mut renderer = Renderer::new(window)?;
        renderer.resize(1024, 768);

        let gizmo = tk3d::agzm::Gizmo::decode(include_bytes!("gizmo.agzm")).unwrap();
        let gizmo_object = Rc::new(GizmoObject {
            mesh: renderer.create_gizmo_mesh(&gizmo.vertices, &gizmo.triangles),
            instances: renderer.create_gizmo_instances(&[GizmoInstance::new(
                Matrix4::from_translation(vec3(0.0, 0.0, -5.0)),
                [1.0; 3],
            )]),
        });

        Ok(Self {
            renderer,
            input: Input::default(),
            gizmo_object,
        })
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<(), ProcessError> {
        self.input.process();
        Ok(())
    }

    pub fn render(&self) -> Result<(), RenderError> {
        let mut scene = Scene::default();
        scene.push_gizmos(self.gizmo_object.clone());

        self.renderer.render(&mut scene)?;
        Ok(())
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
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
