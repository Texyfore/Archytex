mod tools;

use cgmath::vec3;

use crate::graphics::{
    structures::LineVertex, Canvas, Graphics, LineMesh, LineMeshDescriptor, Share,
};

use self::tools::{CameraTool, Tool};

use super::{camera::Camera, elements::ElementKind, input::Input, scene::Scene};

pub struct Editor {
    mode: ElementKind,
    tool: Box<dyn Tool>,
    origin: LineMesh,
}

impl Editor {
    pub fn init(ctx: Context) -> Self {
        Self {
            mode: ElementKind::Solid,
            tool: Box::new(CameraTool),
            origin: ctx.graphics.create_line_mesh(LineMeshDescriptor {
                vertices: &[
                    LineVertex {
                        position: vec3(0.0, 0.0, 0.0),
                        color: [1.0, 0.0, 0.0],
                    },
                    LineVertex {
                        position: vec3(1.0, 0.0, 0.0),
                        color: [1.0, 0.0, 0.0],
                    },
                    LineVertex {
                        position: vec3(0.0, 0.0, 0.0),
                        color: [0.0, 1.0, 0.0],
                    },
                    LineVertex {
                        position: vec3(0.0, 1.0, 0.0),
                        color: [0.0, 1.0, 0.0],
                    },
                    LineVertex {
                        position: vec3(0.0, 0.0, 0.0),
                        color: [0.0, 0.0, 1.0],
                    },
                    LineVertex {
                        position: vec3(0.0, 0.0, 1.0),
                        color: [0.0, 0.0, 1.0],
                    },
                ],
            }),
        }
    }

    pub fn process(&mut self, ctx: Context) {
        let new = self.tool.process(tools::Context {
            input: ctx.input,
            graphics: ctx.graphics,
            camera: ctx.camera,
            scene: ctx.scene,
            delta: ctx.delta,
            mode: self.mode,
        });

        if let Some(new) = new {
            self.tool = new;
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        self.tool.render(canvas);
        canvas.draw_lines(self.origin.share());
    }
}

pub struct Context<'a> {
    pub input: &'a Input,
    pub graphics: &'a Graphics,
    pub camera: &'a mut Camera,
    pub scene: &'a mut Scene,
    pub delta: f32,
}
