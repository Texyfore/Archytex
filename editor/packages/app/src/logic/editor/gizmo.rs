use asset::GizmoID;
use cgmath::{vec3, Deg, InnerSpace, Matrix4, Vector3, Zero};

use crate::{
    graphics::{structures::GizmoInstance, Canvas, GizmoGroup, GizmoInstances, Graphics, Share},
    logic::{camera::Camera, input::Input},
    math::{Aabb, Intersects, Ray},
};

pub struct TranslationGizmo {
    position: Vector3<f32>,
    visible: bool,
    arrows: GizmoInstances,
    sphere: GizmoInstances,
    colliders: [Collider; 3],
}

impl TranslationGizmo {
    pub fn new(graphics: &Graphics) -> Self {
        Self {
            position: Vector3::zero(),
            visible: false,
            arrows: graphics.create_gizmo_instances(3),
            sphere: graphics.create_gizmo_instances(1),
            colliders: [
                Collider::new(vec3(0.55, 0.0, 0.0), vec3(0.45, 0.1, 0.1)),
                Collider::new(vec3(0.0, 0.55, 0.0), vec3(0.1, 0.45, 0.1)),
                Collider::new(vec3(0.0, 0.0, 0.55), vec3(0.1, 0.1, 0.45)),
            ],
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.position = position;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn process(&mut self, graphics: &Graphics, camera: &Camera, input: &Input) {
        if !self.visible {
            return;
        }

        let ray = camera.screen_ray(input.mouse_pos());
        let dist = (self.position - camera.position()).magnitude() * 0.01;

        let mut tints = [Vector3::zero(); 3];

        for (collider, tint) in self.colliders.iter_mut().zip(tints.iter_mut()) {
            collider.raycast(&ray, self.position, 15.0 * dist);
            if collider.collides {
                *tint = vec3(0.5, 0.5, 0.5);
            }
        }

        {
            let translation = Matrix4::from_translation(self.position);
            let scale = Matrix4::from_scale(15.0);
            let rot_x = Matrix4::from_angle_z(Deg(-90.0));
            let rot_z = Matrix4::from_angle_x(Deg(90.0));

            graphics.write_gizmo_instances(
                &self.arrows,
                &[
                    GizmoInstance {
                        matrix: translation * rot_x * scale,
                        color: (vec3(0.5, 0.0, 0.0) + tints[0]).into(),
                    },
                    GizmoInstance {
                        matrix: translation * scale,
                        color: (vec3(0.0, 0.5, 0.0) + tints[1]).into(),
                    },
                    GizmoInstance {
                        matrix: translation * rot_z * scale,
                        color: (vec3(0.0, 0.0, 0.5) + tints[2]).into(),
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

struct Collider {
    aabb: Aabb,
    collides: bool,
}

impl Collider {
    fn new(center: Vector3<f32>, half_extent: Vector3<f32>) -> Self {
        Self {
            aabb: Aabb {
                center,
                half_extent,
            },
            collides: false,
        }
    }

    fn raycast(&mut self, ray: &Ray, position: Vector3<f32>, scale: f32) {
        let aabb = self.aabb.scale_from_origin(scale).translate(position);
        self.collides = ray.intersects(&aabb).is_some();
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
