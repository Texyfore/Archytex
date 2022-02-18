use renderer::Renderer;

use crate::{
    editor::{camera::Camera, scene::Scene},
    input::Input,
};

use super::Tool;

pub struct Context<'a> {
    delta: f32,
    input: &'a Input,
    renderer: &'a Renderer,
    camera: &'a mut Camera,
    scene: &'a mut Scene,
    regen: bool,
    next_tool: Option<Box<dyn Tool>>,
}

impl<'a> Context<'a> {
    pub fn new(
        delta: f32,
        input: &'a Input,
        renderer: &'a Renderer,
        camera: &'a mut Camera,
        scene: &'a mut Scene,
    ) -> Self {
        Self {
            delta,
            input,
            renderer,
            camera,
            scene,
            regen: false,
            next_tool: None,
        }
    }

    pub fn delta(&self) -> f32 {
        self.delta
    }

    pub fn input(&self) -> &Input {
        self.input
    }

    pub fn renderer(&self) -> &Renderer {
        self.renderer
    }

    pub fn camera(&self) -> &Camera {
        self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        self.camera
    }

    pub fn scene(&self) -> &Scene {
        self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        self.scene
    }

    pub fn regen(&self) -> bool {
        self.regen
    }

    pub fn set_regen(&mut self) {
        self.regen = true;
    }

    pub fn take_next_tool(&mut self) -> Option<Box<dyn Tool>> {
        self.next_tool.take()
    }

    pub fn switch_to(&mut self, tool: Box<dyn Tool>) {
        self.next_tool = Some(tool)
    }
}