mod camera;
mod graphics;
mod scene;
mod tools;

use std::marker::PhantomData;

use asset_id::GizmoID;
use renderer::{
    scene::{GizmoObject, Scene as RenderScene},
    Renderer,
};

use crate::input::Input;

use self::{
    camera::Camera,
    graphics::{mesh_gen, Graphics, MeshGenInput},
    scene::Scene,
    tools::{solid, Tool},
};

pub struct Editor {
    camera: Camera,
    scene: Scene,
    tool: Box<dyn Tool>,
    graphics: Option<Graphics>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            scene: Scene::default(),
            tool: Box::new(solid::Select::default()),
            graphics: None,
        }
    }
}

impl Editor {
    pub fn process(&mut self, ctx: Context) {
        let mut tool_ctx =
            tools::Context::new(ctx.delta, ctx.input, &mut self.camera, &mut self.scene);

        self.tool.process(&mut tool_ctx);

        if let Some(next_tool) = tool_ctx.take_next_tool() {
            self.tool.cancelled(&mut tool_ctx);
            self.tool = next_tool;
        }

        if tool_ctx.regen() {
            mesh_gen(
                MeshGenInput {
                    renderer: ctx.renderer,
                    mask: self.tool.graphics_mask(),
                    solids: self.scene.iter_solids(),
                    _f: PhantomData,
                    _p: PhantomData,
                },
                &mut self.graphics,
            );
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
    }

    pub fn window_resized(&mut self, width: u32, height: u32) {
        self.camera.recreate_projection(width, height);
    }
}

pub struct Context<'a> {
    pub delta: f32,
    pub input: &'a Input,
    pub renderer: &'a Renderer,
}
