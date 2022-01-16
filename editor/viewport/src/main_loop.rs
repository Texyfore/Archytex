use std::rc::Rc;

use anyhow::Result;
use renderer::{
    data::{GizmoInstance, GizmoMesh},
    scene::{GizmoObject, Scene},
    Renderer,
};
use tk3d::math::{vec3, Matrix4};
use winit::{
    event::{ElementState, MouseButton, VirtualKeyCode},
    window::Window,
};

use crate::{input::Input, ipc::IpcHost};

pub struct MainLoop {
    renderer: Renderer,
    input: Input,
    gizmo_mesh: Rc<GizmoMesh>,
}

impl MainLoop {
    pub fn new(window: &Window) -> Result<Self> {
        let mut renderer = Renderer::new(window)?;
        renderer.resize(1024, 768);

        let gizmo = tk3d::agzm::Gizmo::decode(include_bytes!("gizmo.agzm")).unwrap();
        let gizmo_mesh = Rc::new(renderer.create_gizmo_mesh(&gizmo.vertices, &gizmo.triangles));

        Ok(Self {
            renderer,
            input: Input::default(),
            gizmo_mesh,
        })
    }

    pub fn process<H: IpcHost>(&mut self, _host: &H) -> Result<()> {
        self.input.process();
        Ok(())
    }

    pub fn render(&self) -> Result<()> {
        let mut scene = Scene::default();
        scene.push_gizmos(GizmoObject {
            mesh: self.gizmo_mesh.clone(),
            instances: Rc::new(self.renderer.create_gizmo_instances(&[
                GizmoInstance::new(
                    Matrix4::from_translation(vec3(-2.0, 0.0, -10.0)),
                    [1.0, 0.0, 0.0],
                ),
                GizmoInstance::new(
                    Matrix4::from_translation(vec3(2.0, 0.0, -10.0)),
                    [0.0, 1.0, 0.0],
                ),
            ])),
        });

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
