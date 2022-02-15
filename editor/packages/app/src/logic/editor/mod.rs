mod tools;

use cgmath::vec3;
use winit::event::VirtualKeyCode;

use crate::graphics::{
    structures::LineVertex, Canvas, Graphics, LineMesh, LineMeshDescriptor, Share,
};

use self::tools::{CameraTool, Tool};

use super::{
    camera::Camera,
    elements::ElementKind,
    input::Input,
    scene::{self, Action, Scene},
};

pub struct Editor {
    mode: ElementKind,
    tool: Box<dyn Tool>,
    origin: LineMesh,
    grid: i32,
}

impl Editor {
    pub fn init(ctx: Context) -> Self {
        Self {
            mode: ElementKind::Solid,
            tool: Box::new(CameraTool::default()),
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
            grid: 100,
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
            grid: self.grid,
        });

        if let Some(new) = new {
            self.tool = new;
        }

        if self.tool.can_switch() {
            for (key, mode) in [
                (VirtualKeyCode::Key1, ElementKind::Solid),
                (VirtualKeyCode::Key2, ElementKind::Face),
                (VirtualKeyCode::Key3, ElementKind::Point),
                (VirtualKeyCode::Key4, ElementKind::Prop),
            ] {
                if ctx.input.is_key_down_once(key) {
                    if self.mode != mode {
                        ctx.scene.act(
                            scene::Context {
                                graphics: ctx.graphics,
                            },
                            Action::DeselectAll(self.mode),
                        );
                        self.mode = mode;
                    }
                    break;
                }
            }
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        self.tool.render(canvas);
        canvas.draw_lines(self.origin.share());
    }

    pub fn mode(&self) -> ElementKind {
        self.mode
    }
}

pub struct Context<'a> {
    pub input: &'a Input,
    pub graphics: &'a Graphics,
    pub camera: &'a mut Camera,
    pub scene: &'a mut Scene,
    pub delta: f32,
}
