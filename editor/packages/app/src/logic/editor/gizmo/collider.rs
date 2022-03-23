use cgmath::{vec3, MetricSpace, Vector2, Vector3, Zero};

use crate::{
    logic::{camera::Camera, common::Axis, editor::gizmo::Selection},
    math::{Aabb, Intersects, Torus2D},
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

pub struct ArcCollider {
    tori: [Torus2D; 3],
}

impl Default for ArcCollider {
    fn default() -> Self {
        Self {
            tori: Axis::all().map(|axis| Torus2D {
                origin: Vector3::zero(),
                normal: axis.unit(),
                inner_radius: 0.35,
                outer_radius: 0.6,
            }),
        }
    }
}

impl ArcCollider {
    pub fn hover_check(&self, info: HoverCheckInfo) -> Option<Axis> {
        struct Hit {
            axis: Axis,
            point: Vector3<f32>,
        }

        let ray = info.camera.screen_ray(info.mouse_position);
        let scale = info.camera.position().distance(info.gizmo_position) * 0.01 * 15.0;
        let mut hits = Vec::new();

        for (torus, axis) in self.tori.iter().zip(Axis::all().into_iter()) {
            let torus = torus
                .scale_from_origin(scale)
                .translate(info.gizmo_position);

            if let Some(intersection) = ray.intersects(&torus) {
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
