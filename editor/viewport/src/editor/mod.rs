mod camera;
mod elements;
mod graphics;
mod scene;
mod tools;
mod grid;

use asset_id::{GizmoID, PropID, TextureID};
use renderer::{
    scene::{GizmoObject, Scene as RenderScene},
    Renderer,
};
use winit::event::VirtualKeyCode;

use crate::{input::Input, ipc::IpcHost};

use self::{
    camera::Camera,
    graphics::Graphics,
    scene::Scene,
    tools::{face, point, solid, Tool},
};

pub struct Editor {
    camera: Camera,
    scene: Scene,
    tool: Box<dyn Tool>,
    graphics: Option<Graphics>,
    grid_step: i32,
    current_texture: TextureID,
    current_prop: PropID,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            scene: Scene::default(),
            tool: Box::new(solid::Hub::default()),
            graphics: None,
            grid_step: 100,
            current_texture: TextureID(0),
            current_prop: PropID(0),
        }
    }
}

impl Editor {
    pub fn process<H: IpcHost>(&mut self, ctx: Context<H>) {
        if ctx.input.is_key_down_once(VirtualKeyCode::Key1) {
            self.change_tool(0);
            ctx.host.send_editor_mode(0);
        } else if ctx.input.is_key_down_once(VirtualKeyCode::Key2) {
            self.change_tool(1);
            ctx.host.send_editor_mode(1);
        } else if ctx.input.is_key_down_once(VirtualKeyCode::Key3) {
            self.change_tool(2);
            ctx.host.send_editor_mode(2);
        }

        let mut tool_ctx = tools::Context::new(
            ctx.delta,
            ctx.input,
            ctx.renderer,
            &mut self.camera,
            &mut self.scene,
        );

        self.tool.process(&mut tool_ctx);

        if let Some(next_tool) = tool_ctx.take_next_tool() {
            self.tool = next_tool;
        }

        if tool_ctx.regen() {
            self.scene
                .regen(ctx.renderer, &mut self.graphics, self.tool.element_mask());
        }
    }

    pub fn render(&self, scene: &mut RenderScene) {
        scene.set_camera_matrices(self.camera.matrix(), self.camera.projection());

        if let Some(graphics) = &self.graphics {
            for mesh_object in &graphics.solid_objects {
                scene.push_solid_object(mesh_object.clone());
            }

            scene.push_line_object(graphics.line_object.clone());
            scene.push_gizmo_object(GizmoObject {
                id: GizmoID(0),
                instances: graphics.point_gizmos.clone(),
            });
        }

        self.tool.render(scene);
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.camera.recreate_projection(width, height);
    }

    pub fn change_tool(&mut self, tool_id: i32) {
        if self.tool.cancellable() {
            match tool_id {
                0 => {
                    self.tool = Box::new(solid::Hub::default());
                }
                1 => {
                    self.tool = Box::new(face::Hub::default());
                }
                2 => {
                    self.tool = Box::new(point::Hub::default());
                }
                _ => {}
            }
        }
    }

    pub fn set_grid_step(&mut self, step: i32) {
        self.grid_step = step;
    }

    pub fn set_camera_speed(&mut self, speed: i32) {
        self.camera.set_speed(speed);
    }

    pub fn set_current_texture(&mut self, texture: TextureID) {
        self.current_texture = texture;
    }

    pub fn set_current_prop(&mut self, prop: PropID) {
        self.current_prop = prop
    }

    pub fn request_camera_speed(&self) -> i32 {
        self.camera.speed()
    }

    pub fn request_grid_step(&self) -> i32 {
        self.grid_step
    }
}

pub struct Context<'a, H: IpcHost> {
    pub delta: f32,
    pub host: &'a H,
    pub input: &'a Input,
    pub renderer: &'a Renderer,
}
