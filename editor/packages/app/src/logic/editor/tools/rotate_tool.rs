use cgmath::{vec3, Vector3, Zero};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    graphics::{structures::LineVertex, Canvas, LineMesh, LineMeshDescriptor, Share},
    logic::elements::{ElementKind, Movable, Prop},
};

use super::{CameraTool, Context, Tool};

pub struct RotateTool {
    props: Vec<(usize, Prop)>,
    originals: Vec<Vector3<i32>>,
    delta: i32,
    orientation: Orientation,
}

impl RotateTool {
    pub fn new(props: Vec<(usize, Prop)>) -> Self {
        let originals = props.iter().map(|(_, prop)| prop.rotation()).collect();
        Self {
            props,
            originals,
            delta: 0,
            orientation: Orientation::Undecided,
        }
    }
}

impl Tool for RotateTool {
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        self.orientation.update(&ctx, &mut self.props);

        if self.orientation.decided() {
            let snap = if ctx.input.is_key_down(VirtualKeyCode::LControl) {
                Snap::Deg15
            } else {
                Snap::None
            };

            let delta = self.delta + ctx.input.mouse_delta().x as i32;

            if delta != self.delta {
                for ((_, prop), original) in self.props.iter_mut().zip(self.originals.iter()) {
                    let snapped = snap.snap(delta);
                    prop.set_rotation(original + self.orientation.delta(snapped));
                    prop.recalc(ctx.graphics);
                }
                self.delta = delta;
            }

            if ctx.input.is_button_down_once(MouseButton::Left) {
                let props = self.props.drain(..).collect();
                Prop::insert_rotate(ctx.scene, props, self.orientation.delta(delta));
                return Some(Box::new(CameraTool::default()));
            }
        }

        if ctx.input.is_button_down_once(MouseButton::Right)
            || ctx.input.is_key_down_once(VirtualKeyCode::R)
            || ctx.input.is_key_down_once(VirtualKeyCode::Escape)
        {
            for ((_, prop), original) in self.props.iter_mut().zip(self.originals.iter()) {
                prop.set_rotation(*original);
                prop.recalc(ctx.graphics);
            }

            let props = self.props.drain(..).collect();
            Prop::insert(ctx.scene, props);
            return Some(Box::new(CameraTool::default()));
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

    fn delta(&self, delta: i32) -> Vector3<i32> {
        match self {
            Self::Undecided => Vector3::zero(),
            Self::Decided { axis, .. } => match axis {
                Axis::X => vec3(delta, 0, 0),
                Axis::Y => vec3(0, delta, 0),
                Axis::Z => vec3(0, 0, delta),
            },
        }
    }
}

enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn unit(&self) -> Vector3<f32> {
        match self {
            Self::X => Vector3::unit_x(),
            Self::Y => Vector3::unit_y(),
            Self::Z => Vector3::unit_z(),
        }
    }

    fn color(&self) -> [f32; 3] {
        match self {
            Self::X => [1.0, 0.0, 0.0],
            Self::Y => [0.0, 1.0, 0.0],
            Self::Z => [0.0, 0.0, 1.0],
        }
    }
}

enum Snap {
    None,
    Deg15,
}

impl Snap {
    fn snap(&self, x: i32) -> i32 {
        match self {
            Snap::None => x,
            Snap::Deg15 => (x as f32 / 15.0) as i32 * 15,
        }
    }
}
