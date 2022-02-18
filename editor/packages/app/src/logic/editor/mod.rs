mod tools;

use asset::{PropID, TextureID};
use cgmath::vec3;
use winit::event::VirtualKeyCode;

use crate::{
    data::PropInfoContainer,
    graphics::{
        structures::GroundVertex, Canvas, Graphics, GroundMesh, GroundMeshDescriptor, Share,
    },
};

use self::tools::{CameraTool, Tool};

use super::{
    camera::Camera,
    elements::ElementKind,
    input::Input,
    scene::{self, Action, Scene},
};

pub struct Editor {
    tool: Box<dyn Tool>,
    ground: Ground,
    mode: ElementKind,
    grid: i32,
    texture: TextureID,
    prop: PropID,
}

impl Editor {
    pub fn init(ctx: Context) -> Self {
        Self {
            tool: Box::new(CameraTool::default()),
            ground: Ground::new(ctx.graphics),
            mode: ElementKind::Solid,
            grid: 100,
            texture: TextureID(2),
            prop: PropID(0),
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
            texture: self.texture,
            prop: self.prop,
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

    pub fn set_texture(&mut self, texture: TextureID) {
        self.texture = texture;
    }

    pub fn set_prop(&mut self, prop: PropID) {
        self.prop = prop;
    }

    pub fn mode(&self) -> ElementKind {
        self.mode
    }

    pub fn render(&self, canvas: &mut Canvas) {
        self.tool.render(canvas);
        self.ground.render(canvas);
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
    mesh: GroundMesh,
}

impl Ground {
    fn new(graphics: &Graphics) -> Self {
        const POSITIONS: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

        let vertices = POSITIONS.map(|pos| GroundVertex {
            position: vec3(pos[0] - 0.5, 0.0, pos[1] - 0.5) * 10000.0,
            texcoord: pos.into(),
        });

        Self {
            mesh: graphics.create_ground_mesh(GroundMeshDescriptor {
                texture: TextureID(1),
                vertices: &vertices,
                triangles: &[[0, 1, 2], [0, 2, 3]],
            }),
        }
    }

    fn render(&self, canvas: &mut Canvas) {
        canvas.draw_ground(self.mesh.share());
    }
}
