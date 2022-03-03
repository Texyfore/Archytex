use cgmath::{Quaternion, Vector2, Vector3, Zero};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    graphics::{structures::LineVertex, Canvas, LineMesh, LineMeshDescriptor, Share},
    logic::{
        editor::common::{calc_angle, Axis, Snap},
        elements::{ElementKind, Movable, Prop},
        scene::{self, Action},
    },
};

use super::{CameraTool, Context, Tool};

pub struct RotateTool {
    origin: Vector2<f32>,
    props: Vec<(usize, Prop)>,
    originals: Vec<Quaternion<f32>>,
    orientation: Orientation,
    angle: i32,
    init_angle: i32,
}

impl RotateTool {
    pub fn new(ctx: &Context, props: Vec<(usize, Prop)>) -> Result<Self, Vec<(usize, Prop)>> {
        let origin = {
            let mut center = Vector3::zero();
            for (_, prop) in &props {
                center += prop.meters();
            }
            center /= props.len() as f32;
            ctx.camera.project(center)
        };

        if let Some(origin) = origin {
            let origin = origin.truncate();
            let originals = props.iter().map(|(_, prop)| prop.rotation()).collect();
            let init_angle = calc_angle(origin, ctx.input.mouse_pos());

            Ok(Self {
                origin,
                props,
                originals,
                orientation: Orientation::Undecided,
                angle: 0,
                init_angle,
            })
        } else {
            Err(props)
        }
    }
}

impl Tool for RotateTool {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        if self.orientation.decided() {
            let snap = if ctx.input.is_key_down(VirtualKeyCode::LControl) {
                Snap::Deg15
            } else {
                Snap::None
            };

            let delta = calc_angle(self.origin, ctx.input.mouse_pos()) - self.init_angle;
            if delta != self.angle {
                for ((_, prop), original) in self.props.iter_mut().zip(self.originals.iter()) {
                    let snapped = snap.snap(delta);
                    prop.set_rotation(self.orientation.angle(snapped) * original);
                    prop.recalc(ctx.graphics);
                }
                self.angle = delta;
            }

            if ctx.input.is_button_down_once(MouseButton::Left) {
                let props = self.props.drain(..).collect();

                ctx.scene
                    .insert_props_with_rotate(props, self.orientation.angle(delta));

                return Some(Box::new(CameraTool::new(ctx.graphics, false)));
            }
        } else {
            self.orientation.update(&ctx, &mut self.props);
            if ctx.input.is_key_down_once(VirtualKeyCode::R) {
                let props = self.props.drain(..).collect::<Vec<_>>();
                let rotations = props
                    .iter()
                    .map(|(index, _)| (*index, Quaternion::new(1.0, 0.0, 0.0, 0.0)))
                    .collect();

                ctx.scene.insert_props(props);
                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::SetPropRotations(rotations),
                );

                return Some(Box::new(CameraTool::new(ctx.graphics, false)));
            }
        }

        if ctx.input.is_button_down_once(MouseButton::Right)
            || ctx.input.is_key_down_once(VirtualKeyCode::Escape)
        {
            for ((_, prop), original) in self.props.iter_mut().zip(self.originals.iter()) {
                prop.set_rotation(*original);
                prop.recalc(ctx.graphics);
            }

            let props = self.props.drain(..).collect();
            Prop::insert(ctx.scene, props);
            return Some(Box::new(CameraTool::new(ctx.graphics, false)));
        }

        None
    }

    fn render(&self, canvas: &mut Canvas) {
        for (_, prop) in &self.props {
            prop.render(canvas, ElementKind::Prop);
        }
        self.orientation.render(canvas);
    }
}

enum Orientation {
    Undecided,
    Decided { axis: Axis, mesh: LineMesh },
}

impl Orientation {
    fn decided(&self) -> bool {
        matches!(self, Self::Decided { .. })
    }

    fn update(&mut self, ctx: &Context, props: &mut [(usize, Prop)]) {
        for (key, axis) in [
            (VirtualKeyCode::X, Axis::X),
            (VirtualKeyCode::Y, Axis::Y),
            (VirtualKeyCode::Z, Axis::Z),
        ] {
            if ctx.input.is_key_down_once(key) {
                let mut vertices = Vec::with_capacity(props.len() * 2);

                for (_, prop) in props {
                    vertices.push(LineVertex {
                        position: prop.meters() - axis.unit() * 10.0,
                        color: axis.color(),
                    });
                    vertices.push(LineVertex {
                        position: prop.meters() + axis.unit() * 10.0,
                        color: axis.color(),
                    });
                    prop.recalc(ctx.graphics);
                }

                *self = Self::Decided {
                    axis,
                    mesh: ctx.graphics.create_line_mesh(LineMeshDescriptor {
                        vertices: &vertices,
                    }),
                };

                return;
            }
        }
    }

    fn render(&self, canvas: &mut Canvas) {
        match self {
            Orientation::Undecided => (),
            Orientation::Decided { mesh, .. } => {
                canvas.draw_lines(mesh.share());
            }
        }
    }

    fn angle(&self, angle: i32) -> Quaternion<f32> {
        match self {
            Self::Undecided => Quaternion::new(1.0, 0.0, 0.0, 0.0),
            Self::Decided { axis, .. } => axis.angle(angle as f32),
        }
    }
}
