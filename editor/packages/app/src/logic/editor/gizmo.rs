use asset::GizmoID;
use cgmath::{Deg, Matrix4, Vector3};

use crate::graphics::{
    structures::GizmoInstance, Canvas, GizmoGroup, GizmoInstances, Graphics, Share,
};

pub struct TranslationGizmo {
    instances: GizmoInstances,
}

impl TranslationGizmo {
    pub fn new(graphics: &Graphics) -> Self {
        Self {
            instances: graphics.create_gizmo_instances(3),
        }
    }

    pub fn set_position(&self, graphics: &Graphics, position: Vector3<f32>) {
        let translation = Matrix4::from_translation(position);
        let scale = Matrix4::from_scale(15.0);
        let rot_x = Matrix4::from_angle_z(Deg(90.0));
        let rot_z = Matrix4::from_angle_x(Deg(90.0));

        graphics.write_gizmo_instances(
            &self.instances,
            &[
                GizmoInstance {
                    matrix: translation * rot_x * scale,
                    color: [1.0, 0.0, 0.0],
                },
                GizmoInstance {
                    matrix: translation * scale,
                    color: [0.0, 1.0, 0.0],
                },
                GizmoInstance {
                    matrix: translation * rot_z * scale,
                    color: [0.0, 0.0, 1.0],
                },
            ],
        );
    }

    pub fn render(&self, canvas: &mut Canvas) {
        canvas.draw_gizmos(GizmoGroup {
            gizmo: GizmoID(1),
            instances: self.instances.share(),
        })
    }
}
