use asset::GizmoID;
use cgmath::{Matrix4, Vector3};

use crate::{
    graphics::{structures::GizmoInstance, Canvas, GizmoGroup, GizmoInstances, Graphics, Share},
    logic::editor::common::Axis,
};

pub struct ArrowGraphics {
    arrows: GizmoInstances,
    sphere: GizmoInstances,
}

impl ArrowGraphics {
    pub fn new_empty(graphics: &Graphics) -> Self {
        Self {
            arrows: graphics.create_gizmo_instances(3),
            sphere: graphics.create_gizmo_instances(1),
        }
    }

    pub fn modify(&self, graphics: &Graphics, position: Vector3<f32>, selected: Option<Axis>) {
        let translation = Matrix4::from_translation(position);
        let arrow_instances = Axis::all()
            .into_iter()
            .map(|axis| {
                let mut color = axis.color();
                if let Some(selected) = selected {
                    if axis == selected {
                        color[0] += 0.1;
                        color[1] += 0.1;
                        color[1] += 0.1;
                    }
                }

                GizmoInstance {
                    matrix: translation * axis.rotation_from_y() * Matrix4::from_scale(15.0),
                    color,
                }
            })
            .collect::<Vec<_>>();

        graphics.write_gizmo_instances(&self.arrows, &arrow_instances);
        graphics.write_gizmo_instances(
            &self.sphere,
            &[GizmoInstance {
                matrix: Matrix4::from_translation(position),
                color: [1.0; 3],
            }],
        );
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_gizmos_no_depth(GizmoGroup {
            gizmo: GizmoID(1),
            instances: self.arrows.share(),
        });

        canvas.draw_gizmos_no_depth(GizmoGroup {
            gizmo: GizmoID(0),
            instances: self.sphere.share(),
        });
    }
}
