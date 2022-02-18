mod tools;

use asset::TextureID;
use cgmath::{vec3, Vector3};
use winit::event::VirtualKeyCode;

use crate::{
    data::PropInfoContainer,
    graphics::{structures::SolidVertex, Canvas, Graphics, Share, SolidMesh, SolidMeshDescriptor},
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
    grid: i32,
    ground: Ground,
}

impl Editor {
    pub fn init(ctx: Context) -> Self {
        Self {
            mode: ElementKind::Solid,
            tool: Box::new(CameraTool::default()),
            grid: 100,
            ground: Ground::new(ctx.graphics),
        }
    }

    pub fn process(&mut self, ctx: Context) {
        let new = self.tool.process(tools::Context {
            input: ctx.input,
            graphics: ctx.graphics,
            prop_infos: ctx.prop_infos,
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
        self.ground.render(canvas);
    }

    pub fn mode(&self) -> ElementKind {
        self.mode
    }
}

pub struct Context<'a> {
    pub input: &'a Input,
    pub graphics: &'a Graphics,
    pub prop_infos: &'a PropInfoContainer,
    pub camera: &'a mut Camera,
    pub scene: &'a mut Scene,
    pub delta: f32,
}

struct Ground {
    mesh: SolidMesh,
}

impl Ground {
    fn new(graphics: &Graphics) -> Self {
        const TINT: [f32; 4] = [0.0; 4];
        const POSITIONS: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let vertices = POSITIONS.map(|pos| SolidVertex {
            position: vec3(pos[0] - 0.5, 0.0, pos[1] - 0.5) * 100.0,
            normal: Vector3::unit_y(),
            texcoord: pos.into(),
            tint: TINT,
        });

        Self {
            mesh: graphics.create_solid_mesh(SolidMeshDescriptor {
                texture: TextureID(0),
                vertices: &vertices,
                triangles: &[[0, 1, 2], [0, 2, 3]],
            }),
        }
    }

    fn render(&self, canvas: &mut Canvas) {
        canvas.draw_solid(self.mesh.share());
    }
}
