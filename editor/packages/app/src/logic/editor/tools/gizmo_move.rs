use cgmath::{Vector3, Zero};
use winit::event::MouseButton;

use crate::{
    graphics::{Canvas, Graphics, LineMesh, LineMeshDescriptor, Share},
    logic::{
        editor::{common::Axis, gizmo::ArrowGraphics, grid},
        elements::Movable,
        ElementKind,
    },
    math::{Ray, Snap},
};

use super::{CameraTool, Context, Tool};

pub struct GizmoMove<E> {
    mask: ElementKind,
    axis: Axis,
    elements: Vec<(usize, E)>,
    center: Vector3<f32>,
    prev_delta: Vector3<i32>,
    correction: Vector3<f32>,
    graphics: ArrowGraphics,
    line: LineMesh,
}

impl<E> GizmoMove<E>
where
    E: Movable,
{
    pub fn new(
        graphics: &Graphics,
        ray: &Ray,
        center: Vector3<f32>,
        mask: ElementKind,
        axis: Axis,
        elements: Vec<(usize, E)>,
    ) -> Self {
        let arrows = ArrowGraphics::new_empty(graphics);
        arrows.modify(graphics, center, Some(axis), true);

        let line = graphics.create_line_mesh(LineMeshDescriptor {
            vertices: &axis.line_vertices(center),
        });

        Self {
            mask,
            axis,
            elements,
            center,
            prev_delta: Vector3::zero(),
            correction: ray.closest_point_on_line(center, axis.unit()),
            graphics: arrows,
            line,
        }
    }
}

impl<E> Tool for GizmoMove<E>
where
    E: Movable,
{
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        let ray = ctx.camera.screen_ray(ctx.input.mouse_pos());
        let point = ray.closest_point_on_line(self.center, self.axis.unit()) - self.correction;
        let delta = point.snap(grid(*ctx.grid));
        if delta != self.prev_delta {
            let delta2 = delta - self.prev_delta;
            self.prev_delta = delta;

            for (_, element) in &mut self.elements {
                element.displace(delta2, self.mask);
                element.recalc(ctx.graphics);
            }

            self.graphics.modify(
                ctx.graphics,
                self.center + delta.map(|e| e as f32 * 0.01),
                Some(self.axis),
                true,
            );
        }

        if ctx.input.was_button_down_once(MouseButton::Left) {
            let elements = self.elements.drain(..).collect();
            E::insert_move(ctx.scene, elements, delta, self.mask);
            return Some(Box::new(CameraTool::new(ctx.graphics, false)));
        }

        None
    }

    fn render(&self, canvas: &mut Canvas) {
        self.graphics.render(canvas);
        canvas.draw_lines(self.line.share());

        for (_, element) in &self.elements {
            element.render(canvas, self.mask);
        }
    }

    fn keep_old(&self) -> bool {
        true
    }
}
