mod gizmo;
mod tools;

use asset::{PropID, TextureID};
use cgmath::vec3;
use winit::event::VirtualKeyCode;

use crate::{
    button,
    data::PropInfoContainer,
    graphics::{
        structures::GroundVertex, Canvas, Graphics, GroundMesh, GroundMeshDescriptor, Share,
    },
    Host, ToHost,
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
    old_tool: Option<Box<dyn Tool>>,
    ground: Ground,
    mode: ElementKind,
    grid: i32,
    texture: TextureID,
    prop: PropID,
}

impl Editor {
    pub fn init(ctx: Context) -> Self {
        Self {
            tool: Box::new(CameraTool::new(ctx.graphics)),
            old_tool: None,
            ground: Ground::new(ctx.graphics),
            mode: ElementKind::Solid,
            grid: 128,
            texture: TextureID(2),
            prop: PropID(0),
        }
    }

    pub fn process(&mut self, ctx: Context) {
        self.old_tool = None;

        let new = self.tool.process(tools::Context {
            input: ctx.input,
            graphics: ctx.graphics,
            prop_infos: ctx.prop_infos,
            camera: ctx.camera,
            scene: ctx.scene,
            delta: ctx.delta,
            mode: self.mode,
            grid: &mut self.grid,
            texture: self.texture,
            prop: self.prop,
        });

        if let Some(new) = new {
            if self.tool.keep_old() {
                self.old_tool = Some(std::mem::replace(&mut self.tool, new));
            } else {
                self.tool = new;
            }
        }

        if self.tool.can_switch() {
            for (key, mode, button) in [
                (VirtualKeyCode::Key1, ElementKind::Solid, button::SOLID),
                (VirtualKeyCode::Key2, ElementKind::Face, button::FACE),
                (VirtualKeyCode::Key3, ElementKind::Point, button::POINT),
                (VirtualKeyCode::Key4, ElementKind::Prop, button::PROP),
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
                        ctx.host.callback(ToHost::Button(button));
                        println!("[wasm] button {}", button);
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

    pub fn set_mode(&mut self, ctx: Context, mode: ElementKind) {
        if self.mode != mode {
            ctx.scene.act(
                scene::Context {
                    graphics: ctx.graphics,
                },
                Action::DeselectAll(self.mode),
            );
            self.mode = mode;
        }
    }

    pub fn mode(&self) -> ElementKind {
        self.mode
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.set_grid_len(self.grid);

        let tool = if let Some(old) = &self.old_tool {
            old
        } else {
            &self.tool
        };

        tool.render(canvas);
        self.ground.render(canvas);
    }
}

pub struct Context<'a> {
    pub host: &'a dyn Host,
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
        let mesh = {
            const POSITIONS: [[f32; 2]; 4] = [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

            let vertices = POSITIONS.map(|pos| GroundVertex {
                position: vec3(pos[0] - 0.5, 0.0, pos[1] - 0.5) * 500.0,
                texcoord: pos.into(),
            });

            graphics.create_ground_mesh(GroundMeshDescriptor {
                texture: TextureID(1),
                vertices: &vertices,
                triangles: &[[0, 1, 2], [0, 2, 3]],
            })
        };

        Self { mesh }
    }

    fn render(&self, canvas: &mut Canvas) {
        canvas.draw_ground(self.mesh.share());
    }
}
