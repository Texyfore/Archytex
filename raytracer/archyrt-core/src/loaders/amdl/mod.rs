use crate::intersectables::triangle::Triangle;
use crate::loaders::Loader;
use crate::utilities::math::{Vec2, Vec3};
use crate::{cameras::perspective::PerspectiveCamera, vector};
use anyhow::{anyhow, Result};
use mdl;
use mdl::{Face, Point, Vector2};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Mul;
use std::path::Path;

pub struct AMDLLoader {
    triangles: Vec<Triangle>,
    camera: PerspectiveCamera,
}

fn to_hashmap<T>(a: Vec<(u32, T)>) -> HashMap<u32, T> {
    let mut map = HashMap::new();
    for (k, v) in a {
        map.insert(k, v);
    }
    map
}

fn texcoord(point: Vec3, normal: Vec3) -> Vec2 {
    if normal.x().abs() > normal.y().abs() {
        if normal.x().abs() > normal.z().abs() {
            vector![point.y(), point.z()]
        } else {
            vector![point.x(), point.y()]
        }
    } else if normal.y().abs() > normal.z().abs() {
        vector![point.x(), point.z()]
    } else {
        vector![point.x(), point.y()]
    }
}

impl AMDLLoader {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf)?;
        return Self::from_bytes(buf);
    }
    pub fn from_bytes(data: Vec<u8>) -> Result<Self> {
        let scene =
            mdl::Scene::decode(data.as_slice()).ok_or(anyhow!("Unable to decode scene file"))?;
        return Self::from_scene(scene);
    }

    pub fn from_scene(scene: mdl::Scene) -> Result<Self> {
        let mut triangles: Vec<Triangle> = Vec::new();
        let focal_distance = 0.595877;
        let camera = PerspectiveCamera::from_euler(
            scene.camera.position.into(),
            scene.camera.rotation.into(),
            focal_distance,
        );
        let faces = to_hashmap(scene.model.faces);
        let points = to_hashmap(scene.model.points);
        for (_, solid) in &scene.model.solids[1..] {
            let faces: Vec<_> = solid
                .faces
                .iter()
                .map(|id| faces.get(id))
                .into_iter()
                .flatten()
                .collect();
            if faces.len() != 6 {
                return Err(anyhow!("Invalid face ID"));
            }
            for face in faces {
                //Counterclockwise
                let points: Vec<_> = face
                    .points
                    .iter()
                    .map(|id| points.get(id))
                    .into_iter()
                    .flatten()
                    .collect();
                if points.len() != 4 {
                    return Err(anyhow!("Invalid face ID"));
                }
                let point_positions: Vec<Vec3> = points
                    .iter()
                    .map(|point| (&point.position).into())
                    .collect();
                let edge0 = point_positions[1] - point_positions[0];
                let edge1 = point_positions[3] - point_positions[0];
                let normal = edge0.cross(edge1).normalized();

                let uv0 = texcoord(point_positions[0], normal);
                let uv1 = texcoord(point_positions[1], normal);
                let uv2 = texcoord(point_positions[2], normal);
                let uv3 = texcoord(point_positions[3], normal);

                let point0 = {
                    let mut a = point_positions[0];
                    a[2] = -a[2];
                    a
                };
                let point1 = {
                    let mut a = point_positions[1];
                    a[2] = -a[2];
                    a
                };
                let point2 = {
                    let mut a = point_positions[2];
                    a[2] = -a[2];
                    a
                };
                let point3 = {
                    let mut a = point_positions[3];
                    a[2] = -a[2];
                    a
                };
                let triangle1 = Triangle::new(
                    [point0, point2, point1],
                    [uv0, uv2, uv1],
                    Vec3::from_single(0.5),
                );
                let triangle2 = Triangle::new(
                    [point0, point3, point2],
                    [uv0, uv3, uv2],
                    Vec3::from_single(0.5),
                );
                triangles.push(triangle1);
                triangles.push(triangle2);
            }
        }
        Ok(Self { camera, triangles })
    }
}

impl Loader for AMDLLoader {
    type C = PerspectiveCamera;

    fn get_triangles(&self) -> &Vec<Triangle> {
        return &self.triangles;
    }

    fn get_camera(&self) -> &Self::C {
        return &self.camera;
    }
}
