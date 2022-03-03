use cgmath::{vec3, MetricSpace, Vector2, Vector3};

use crate::{
    logic::{
        camera::Camera,
        editor::{common::Axis, gizmo::Selection},
    },
    math::{Aabb, Intersects},
};

pub struct ArrowCollider {
    axis_boxes: [Aabb; 3],
    plane_boxes: [Aabb; 3],
}

impl Default for ArrowCollider {
    fn default() -> Self {
        Self {
            axis_boxes: [
                Aabb {
                    center: vec3(0.55, 0.0, 0.0),
                    half_extent: vec3(0.45, 0.1, 0.1),
                },
                Aabb {
                    center: vec3(0.0, 0.55, 0.0),
                    half_extent: vec3(0.1, 0.45, 0.1),
                },
                Aabb {
                    center: vec3(0.0, 0.0, 0.55),
                    half_extent: vec3(0.1, 0.1, 0.45),
                },
            ],
            plane_boxes: [
                Aabb {
                    center: vec3(0.0, 0.7, 0.7),
                    half_extent: vec3(0.02, 0.2, 0.2),
                },
                Aabb {
                    center: vec3(0.7, 0.0, 0.7),
                    half_extent: vec3(0.2, 0.02, 0.2),
                },
                Aabb {
                    center: vec3(0.7, 0.7, 0.0),
                    half_extent: vec3(0.2, 0.2, 0.02),
                },
            ],
        }
    }
}

impl ArrowCollider {
    pub fn hover_check(&self, info: HoverCheckInfo) -> Option<Selection> {
        struct Hit {
            selection: Selection,
            point: Vector3<f32>,
        }

        let ray = info.camera.screen_ray(info.mouse_position);
        let scale = info.camera.position().distance(info.gizmo_position) * 0.01 * 15.0;
        let mut hits = Vec::new();

        for (aabb, axis) in self.axis_boxes.iter().zip(Axis::all().into_iter()) {
            let aabb = aabb.scale_from_origin(scale).translate(info.gizmo_position);
            if let Some(intersection) = ray.intersects(&aabb) {
                hits.push(Hit {
                    selection: Selection::Axis(axis),
                    point: intersection.point,
                });
            }
        }

        for (aabb, axis) in self.plane_boxes.iter().zip(Axis::all().into_iter()) {
            let aabb = aabb.scale_from_origin(scale).translate(info.gizmo_position);
            if let Some(intersection) = ray.intersects(&aabb) {
                hits.push(Hit {
                    selection: Selection::Plane(axis),
                    point: intersection.point,
                });
            }
        }

        hits.sort_unstable_by(|a, b| {
            let dist_a = ray.start.distance2(a.point);
            let dist_b = ray.start.distance2(b.point);
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        hits.first().map(|hit| hit.selection)
    }
}

pub struct HoverCheckInfo<'a> {
    pub camera: &'a Camera,
    pub mouse_position: Vector2<f32>,
    pub gizmo_position: Vector3<f32>,
}
