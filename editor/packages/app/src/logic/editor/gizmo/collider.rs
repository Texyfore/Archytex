use cgmath::{vec3, MetricSpace, Vector2, Vector3};

use crate::{
    logic::{camera::Camera, editor::common::Axis},
    math::{Aabb, Intersects},
};

pub struct ArrowCollider {
    boxes: [Aabb; 3],
}

impl Default for ArrowCollider {
    fn default() -> Self {
        Self {
            boxes: [
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
        }
    }
}

impl ArrowCollider {
    pub fn axis_above_cursor(&self, info: HoverCheckInfo) -> Option<Axis> {
        struct Hit {
            axis: Axis,
            point: Vector3<f32>,
        }

        let ray = info.camera.screen_ray(info.mouse_position);
        let scale = info.camera.position().distance(info.gizmo_position) * 0.01 * 15.0;
        let mut hits = Vec::new();

        for (aabb, axis) in self.boxes.iter().zip(Axis::all().into_iter()) {
            let aabb = aabb.scale_from_origin(scale).translate(info.gizmo_position);
            if let Some(intersection) = ray.intersects(&aabb) {
                hits.push(Hit {
                    axis,
                    point: intersection.point,
                });
            }
        }

        hits.sort_unstable_by(|a, b| {
            let dist_a = ray.start.distance2(a.point);
            let dist_b = ray.start.distance2(b.point);
            dist_a.partial_cmp(&dist_b).unwrap()
        });

        hits.first().map(|hit| hit.axis)
    }
}

pub struct HoverCheckInfo<'a> {
    pub camera: &'a Camera,
    pub mouse_position: Vector2<f32>,
    pub gizmo_position: Vector3<f32>,
}
