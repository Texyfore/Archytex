use std::collections::HashMap;

use cgmath::{InnerSpace, MetricSpace, Vector2, Vector3, Zero};

use crate::{
    data::PropInfoContainer,
    logic::camera::Camera,
    math::{Intersects, Plane, Ray, Sphere, Triangle},
};

use super::{FaceLocator, PointLocator, Prop, Solid};

pub fn raycast(input: RaycastInput) -> RaycastHit {
    let ray = input.camera.screen_ray(input.screen_pos);

    let mut hit_faces = raycast_faces(input.solids, &ray);
    let mut hit_props = raycast_props(input.props, input.prop_infos, &ray);
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

    hit_props.sort_unstable_by(|a, b| {
        let dist_a = a.point.distance2(ray.start);
        let dist_b = b.point.distance2(ray.start);
        dist_a.partial_cmp(&dist_b).unwrap()
    });

    let firsts = (hit_faces.first(), hit_props.first());
    let endpoint = match firsts {
        (Some(face), None) => Some(RaycastEndpoint {
            point: face.point,
            normal: face.normal,
            kind: RaycastEndpointKind::Face(face.locator),
        }),
        (None, Some(prop)) => Some(RaycastEndpoint {
            point: prop.point,
            normal: Vector3::zero(),
            kind: RaycastEndpointKind::Prop(prop.index),
        }),
        (Some(face), Some(prop)) => {
            let dist_face = face.point.distance2(ray.start);
            let dist_prop = prop.point.distance2(ray.start);

            if dist_face < dist_prop {
                Some(RaycastEndpoint {
                    point: face.point,
                    normal: face.normal,
                    kind: RaycastEndpointKind::Face(face.locator),
                })
            } else {
                Some(RaycastEndpoint {
                    point: prop.point,
                    normal: Vector3::zero(),
                    kind: RaycastEndpointKind::Prop(prop.index),
                })
            }
        }
        (None, None) => {
            let plane = Plane {
                origin: Vector3::zero(),
                normal: Vector3::unit_y(),
            };

            ray.intersects(&plane).map(|intersection| RaycastEndpoint {
                point: intersection.point,
                normal: intersection.normal,
                kind: RaycastEndpointKind::Ground,
            })
        }
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

fn raycast_props(
    props: &HashMap<usize, Prop>,
    infos: &PropInfoContainer,
    ray: &Ray,
) -> Vec<HitProp> {
    let mut hit_props = Vec::new();

    for (index, prop) in props {
        if let Some(point) = prop.intersects(infos, ray) {
            hit_props.push(HitProp {
                index: *index,
                point,
            });
        }
    }

    hit_props
}

pub struct RaycastInput<'a> {
    pub solids: &'a HashMap<usize, Solid>,
    pub props: &'a HashMap<usize, Prop>,
    pub camera: &'a Camera,
    pub prop_infos: &'a PropInfoContainer,
    pub screen_pos: Vector2<f32>,
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

struct HitProp {
    index: usize,
    point: Vector3<f32>,
}
