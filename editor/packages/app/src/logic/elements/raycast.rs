use std::collections::HashMap;

use asset::Prop;
use cgmath::{InnerSpace, MetricSpace, Vector2, Vector3, Zero};

use crate::{
    logic::camera::Camera,
    math::{Intersects, Plane, Ray, Sphere, Triangle},
};

use super::Solid;

pub fn raycast(input: RaycastInput) -> RaycastHit {
    let ray = input.camera.screen_ray(input.screen_pos);

    let mut hit_faces = raycast_faces(input.solids, &ray);
    let mut hit_points = Vec::new();

    for (index, solid) in input.solids {
        let geometry = &solid.geometry;
        for (i, point) in geometry.points.iter().enumerate() {
            let origin = point.meters();
            let dist = origin.distance(ray.start);

            let sphere = Sphere {
                origin,
                radius: dist * 0.1,
            };

            if ray.intersects(&sphere).is_some() {
                if let Some(screen) = input.camera.project(origin) {
                    let screen = screen.truncate();
                    hit_points.push(HitPoint {
                        locator: PointLocator {
                            solid: *index,
                            point: i,
                        },
                        world: origin,
                        screen,
                    });
                }
            }
        }
    }

    hit_faces.sort_unstable_by(|a, b| {
        let dist_a = a.point.distance2(ray.start);
        let dist_b = b.point.distance2(ray.start);
        dist_a.partial_cmp(&dist_b).unwrap()
    });

    let endpoint = if let Some(hit_face) = hit_faces.first() {
        Some(RaycastEndpoint {
            point: hit_face.point,
            normal: hit_face.normal,
            kind: RaycastEndpointKind::Face(hit_face.locator),
        })
    } else {
        let plane = Plane {
            origin: Vector3::zero(),
            normal: Vector3::unit_y(),
        };

        ray.intersects(&plane).map(|intersection| RaycastEndpoint {
            point: intersection.point,
            normal: intersection.normal,
            kind: RaycastEndpointKind::Ground,
        })
    };

    hit_points.sort_unstable_by(|a, b| {
        let dist_a = input.screen_pos.distance2(a.screen);
        let dist_b = input.screen_pos.distance2(b.screen);
        dist_a.partial_cmp(&dist_b).unwrap()
    });

    if !hit_points.is_empty() {
        let first = hit_points[0].screen;
        hit_points.retain(|hit| {
            input
                .camera
                .project(hit.world)
                .unwrap()
                .truncate()
                .distance2(first)
                < 25.0
        });
    }

    hit_points.retain(|hit| {
        let ray = input.camera.screen_ray(hit.screen);
        let dist = (hit.world - ray.start).magnitude2() * 0.99;
        raycast_faces(input.solids, &ray)
            .iter()
            .all(|hit| hit.point.distance2(ray.start) > dist)
    });

    RaycastHit {
        endpoint,
        points: hit_points
            .into_iter()
            .map(|hit_point| hit_point.locator)
            .collect(),
    }
}

fn raycast_faces(solids: &HashMap<usize, Solid>, ray: &Ray) -> Vec<HitFace> {
    let mut hit_faces = Vec::new();

    for (index, solid) in solids {
        let geometry = &solid.geometry;
        for (i, face) in geometry.faces.iter().enumerate() {
            let triangles = [
                Triangle {
                    a: geometry.points[face.indices[0]].meters(),
                    b: geometry.points[face.indices[1]].meters(),
                    c: geometry.points[face.indices[2]].meters(),
                },
                Triangle {
                    a: geometry.points[face.indices[0]].meters(),
                    b: geometry.points[face.indices[2]].meters(),
                    c: geometry.points[face.indices[3]].meters(),
                },
            ];

            for triangle in triangles {
                if let Some(intersection) = ray.intersects(&triangle) {
                    hit_faces.push(HitFace {
                        locator: FaceLocator {
                            solid: *index,
                            face: i,
                        },
                        point: intersection.point,
                        normal: intersection.normal,
                    });
                    break;
                }
            }
        }
    }

    hit_faces
}

pub struct RaycastInput<'a> {
    solids: &'a HashMap<usize, Solid>,
    props: &'a HashMap<usize, Prop>,
    camera: &'a Camera,
    screen_pos: Vector2<f32>,
}

pub struct RaycastHit {
    pub endpoint: Option<RaycastEndpoint>,
    pub points: Vec<PointLocator>,
}

pub struct RaycastEndpoint {
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub kind: RaycastEndpointKind,
}

pub enum RaycastEndpointKind {
    Face(FaceLocator),
    Prop(usize),
    Ground,
}

#[derive(Clone, Copy)]
pub struct PointLocator {
    pub solid: usize,
    pub point: usize,
}

#[derive(Clone, Copy)]
pub struct FaceLocator {
    pub solid: usize,
    pub face: usize,
}

struct HitPoint {
    locator: PointLocator,
    world: Vector3<f32>,
    screen: Vector2<f32>,
}

struct HitFace {
    locator: FaceLocator,
    point: Vector3<f32>,
    normal: Vector3<f32>,
}
