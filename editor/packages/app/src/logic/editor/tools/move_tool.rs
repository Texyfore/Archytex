use cgmath::{vec3, Vector3, Zero};
use winit::event::{MouseButton, VirtualKeyCode};

use crate::{
    graphics::{Canvas, LineMesh, LineMeshDescriptor, Share},
    logic::{
        common::Axis,
        elements::{ElementKind, Movable},
        scene::{self, Action},
    },
    math::{Intersects, Plane, Ray, Snap},
};

use super::{CameraTool, Context, Tool};

pub struct MoveTool<E> {
    mask: ElementKind,
    clone: bool,

    ray: Ray,
    center: Vector3<f32>,
    plane: Plane,
    elements: Vec<(usize, E)>,

    start: Vector3<f32>,
    delta: Vector3<i32>,
    snap: MoveSnap,
    line_mesh: Option<LineMesh>,
}

impl<E> MoveTool<E>
where
    E: Movable,
{
    pub fn new(
        mask: ElementKind,
        ray: Ray,
        elements: Vec<(usize, E)>,
        clone: bool,
    ) -> Result<Self, Vec<(usize, E)>> {
        let dir = ray.direction();
        let normal = -dir;

        let mut center = Vector3::zero();
        for (_, element) in &elements {
            center += element.center(mask);
        }
        center /= elements.len() as f32;

        let plane = Plane {
            origin: center,
            normal,
        };

        if let Some(intersection) = ray.intersects(&plane) {
            Ok(Self {
                mask,
                clone,
                ray,
                center,
                plane,
                elements,
                start: intersection.point + intersection.normal * 0.0001,
                delta: Vector3::zero(),
                snap: MoveSnap::None,
                line_mesh: None,
            })
        } else {
            Err(elements)
        }
    }

    fn snap_to_axis(&mut self, ctx: &Context, axis: Axis) {
        let normal = -self.ray.direction();
        self.plane = Plane {
            origin: self.center,
            normal: match axis {
                Axis::Y => vec3(normal.x, 0.0, normal.z),
                _ => Vector3::unit_y(),
            },
        };

        self.snap = MoveSnap::Axis(axis);

        if let Some(intersection) = self.ray.intersects(&self.plane) {
            self.start = intersection.point + intersection.normal * 0.0001;
        }

        self.line_mesh = Some(ctx.graphics.create_line_mesh(LineMeshDescriptor {
            vertices: &axis.line_vertices(self.center),
        }));
    }

    fn snap_exclude(&mut self, ctx: &Context, axis: Axis) {
        self.plane = Plane {
            origin: self.center,
            normal: axis.unit(),
        };

        self.snap = MoveSnap::Plane(axis);

        if let Some(intersection) = self.ray.intersects(&self.plane) {
            self.start = intersection.point + intersection.normal * 0.001;
        }

        let verts = axis
            .others()
            .into_iter()
            .flat_map(|axis| axis.line_vertices(self.center))
            .collect::<Vec<_>>();

        self.line_mesh = Some(
            ctx.graphics
                .create_line_mesh(LineMeshDescriptor { vertices: &verts }),
        );
    }
}

impl<E> Tool for MoveTool<E>
where
    E: Movable,
{
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        for (axis, key) in [
            (Axis::X, VirtualKeyCode::X),
            (Axis::Y, VirtualKeyCode::Z),
            (Axis::Z, VirtualKeyCode::Y),
        ] {
            if ctx.input.is_key_down_once(key) {
                if ctx.input.is_key_down(VirtualKeyCode::LShift) {
                    self.snap_exclude(&ctx, axis);
                } else {
                    self.snap_to_axis(&ctx, axis);
                }
            }
        }

        let mouse_pos = ctx.input.mouse_pos();
        let ray = ctx.camera.screen_ray(mouse_pos);

        if let Some(intersection) = ray.intersects(&self.plane) {
            let start = self.start.snap(*ctx.grid);
            let end = (intersection.point + intersection.normal * 0.001).snap(*ctx.grid);

            let delta = self.snap.snap_vec(end - start);
            if delta != self.delta {
                for (_, element) in &mut self.elements {
                    element.displace(delta - self.delta, self.mask);
                    element.recalc(ctx.graphics);
                }
                self.delta = delta;
            }
        }

        if ctx.input.was_button_down_once(MouseButton::Left) {
            let elements = self.elements.drain(..).collect::<Vec<_>>();

            if self.clone {
                E::insert_remove(ctx.scene, elements);
                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::DeselectAll(self.mask),
                );
            } else {
                E::insert_move(ctx.scene, elements, self.delta, self.mask);
            }

            return Some(Box::new(CameraTool::new(ctx.graphics)));
        }

        if ctx.input.is_button_down_once(MouseButton::Right)
            || ctx.input.is_key_down_once(VirtualKeyCode::G)
            || ctx.input.is_key_down_once(VirtualKeyCode::Escape)
        {
            let mut elements = self.elements.drain(..).collect::<Vec<_>>();

            if !self.clone {
                for (_, element) in &mut elements {
                    element.displace(-self.delta, self.mask);
                    element.recalc(ctx.graphics);
                }

                E::insert(ctx.scene, elements);
            } else {
                ctx.scene.act(
                    scene::Context {
                        graphics: ctx.graphics,
                    },
                    Action::DeselectAll(self.mask),
                );
            }

            return Some(Box::new(CameraTool::new(ctx.graphics)));
        }

        None
    }

    fn render(&self, canvas: &mut Canvas) {
        for (_, element) in &self.elements {
            element.render(canvas, self.mask);
        }

        if let Some(line_mesh) = &self.line_mesh {
            canvas.draw_lines(line_mesh.share());
        }
    }
}

enum MoveSnap {
    None,
    Axis(Axis),
    Plane(Axis),
}

impl MoveSnap {
    fn snap_vec(&self, vec: Vector3<i32>) -> Vector3<i32> {
        match self {
            MoveSnap::None => vec,
            MoveSnap::Axis(axis) => match axis {
                Axis::X => vec3(vec.x, 0, 0),
                Axis::Y => vec3(0, vec.y, 0),
                Axis::Z => vec3(0, 0, vec.z),
            },
            MoveSnap::Plane(axis) => match axis {
                Axis::X => vec3(0, vec.y, vec.z),
                Axis::Y => vec3(vec.x, 0, vec.z),
                Axis::Z => vec3(vec.x, vec.y, 0),
            },
        }
    }
}
