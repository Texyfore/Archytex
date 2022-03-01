use asset::GizmoID;
use cgmath::{Deg, Matrix4, Vector3};

use crate::graphics::{
    structures::GizmoInstance, Canvas, GizmoGroup, GizmoInstances, Graphics, Share,
};

pub struct TranslationGizmo {
    arrows: GizmoInstances,
    sphere: GizmoInstances,
    visible: bool,
}

impl TranslationGizmo {
    pub fn new(graphics: &Graphics) -> Self {
        Self {
            arrows: graphics.create_gizmo_instances(3),
            sphere: graphics.create_gizmo_instances(1),
            visible: false,
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn set_position(&self, graphics: &Graphics, position: Vector3<f32>) {
        let translation = Matrix4::from_translation(position);
        let scale = Matrix4::from_scale(15.0);
        let rot_x = Matrix4::from_angle_z(Deg(-90.0));
        let rot_z = Matrix4::from_angle_x(Deg(90.0));

        graphics.write_gizmo_instances(
            &self.arrows,
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

        graphics.write_gizmo_instances(
            &self.sphere,
            &[GizmoInstance {
                matrix: translation * Matrix4::from_scale(1.5),
                color: [1.0; 3],
            }],
        );
    }

    pub fn render(&self, canvas: &mut Canvas) {
        if !self.visible {
            return;
        }

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

// pub struct RotationGizmo {
//     instances: GizmoInstances,
// }

// impl RotationGizmo {
//     pub fn new(graphics: &Graphics) -> Self {
//         Self {
//             instances: graphics.create_gizmo_instances(3),
//         }
//     }

//     pub fn set_position(&self, graphics: &Graphics, position: Vector3<f32>) {
//         let translation = Matrix4::from_translation(position);
//         let scale = Matrix4::from_scale(15.0);
//         let rot_y = Matrix4::from_angle_y(Deg(-90.0));
//         let rot_x = Matrix4::from_angle_y(Deg(180.0)) * Matrix4::from_angle_z(Deg(90.0));
//         let rot_z = Matrix4::from_angle_x(Deg(90.0));

//         graphics.write_gizmo_instances(
//             &self.instances,
//             &[
//                 GizmoInstance {
//                     matrix: translation * rot_x * scale,
//                     color: [1.0, 0.0, 0.0],
//                 },
//                 GizmoInstance {
//                     matrix: translation * rot_y * scale,
//                     color: [0.0, 1.0, 0.0],
//                 },
//                 GizmoInstance {
//                     matrix: translation * rot_z * scale,
//                     color: [0.0, 0.0, 1.0],
//                 },
//             ],
//         );
//     }

//     pub fn render(&self, canvas: &mut Canvas) {
//         canvas.draw_gizmos_no_depth(GizmoGroup {
//             gizmo: GizmoID(2),
//             instances: self.instances.share(),
//         })
//     }
// }
