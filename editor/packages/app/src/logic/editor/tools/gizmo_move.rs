use cgmath::{Vector3, Zero};
use winit::event::MouseButton;

use crate::{
    graphics::{Canvas, Graphics, LineMesh, Share},
    logic::{
        editor::gizmo::{ArrowGraphics, Selection},
        elements::Movable,
        ElementKind,
    },
    math::{Intersects, Plane, Ray, Snap},
};

use super::{CameraTool, Context, Tool};

pub struct GizmoMove<E> {
    mask: ElementKind,
    selection: Selection,
    elements: Vec<(usize, E)>,
    center: Vector3<f32>,
    prev_delta: Vector3<i32>,
    correction: Vector3<f32>,
    plane: Option<Plane>,
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
        selection: Selection,
        elements: Vec<(usize, E)>,
    ) -> Result<Self, Vec<(usize, E)>> {
        let arrows = ArrowGraphics::new_empty(graphics);
        arrows.modify(graphics, center, Some(selection), true);

        let verts = selection.line_vertices(center);
        let line = graphics.create_line_mesh_uninit(verts.len());
        graphics.write_line_mesh(&line, &verts);

        let (correction, plane) = if selection.is_axis() {
            (
                ray.closest_point_on_line(center, selection.axis().unit()),
                None,
            )
        } else {
            let plane = Plane {
                origin: center,
                normal: selection.axis().unit(),
            };

            if let Some(intersection) = ray.intersects(&plane) {
                let correction = intersection.point;
                (correction, Some(plane))
            } else {
                return Err(elements);
            }
        };

        Ok(Self {
            mask,
            selection,
            elements,
            center,
            prev_delta: Vector3::zero(),
            correction,
            plane,
            graphics: arrows,
            line,
        })
    }
}

impl<E> Tool for GizmoMove<E>
where
    E: Movable,
{
    fn process(&mut self, ctx: Context) -> Option<Box<dyn Tool>> {
        let ray = ctx.camera.screen_ray(ctx.input.mouse_pos());

        let point = if let Some(plane) = &self.plane {
            let ray = ctx.camera.screen_ray(ctx.input.mouse_pos());
            if let Some(intersection) = ray.intersects(plane) {
                intersection.point - self.correction + intersection.normal * 0.001
            } else {
                return None;
            }
        } else {
            ray.closest_point_on_line(self.center, self.selection.axis().unit()) - self.correction
        };

        let delta = point.snap(*ctx.grid);
        if delta != self.prev_delta {
            let delta2 = delta - self.prev_delta;
            self.prev_delta = delta;

            let verts = self
                .selection
                .line_vertices(self.center + delta.map(|e| e as f32 / 128.0));

            ctx.graphics.write_line_mesh(&self.line, &verts);

            for (_, element) in &mut self.elements {
                element.displace(delta2, self.mask);
                element.recalc(ctx.graphics);
            }

            self.graphics.modify(
                ctx.graphics,
                self.center + delta.map(|e| e as f32 / 128.0),
                Some(self.selection),
                true,
            );
        }

        if ctx.input.was_button_down_once(MouseButton::Left) {
            let elements = self.elements.drain(..).collect();
            E::insert_move(ctx.scene, elements, delta, self.mask);
            return Some(Box::new(CameraTool::new(ctx.graphics)));
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
