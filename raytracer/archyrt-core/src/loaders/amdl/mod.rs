use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::ops::Mul;
use std::path::Path;
use crate::cameras::perspective::PerspectiveCamera;
use crate::intersectables::triangle::Triangle;
use crate::loaders::Loader;
use mdl;
use anyhow::{anyhow, Result};
use mdl::{Face, Point, Vector2};
use crate::utilities::math::Vec3;

pub struct AMDLLoader{
    triangles: Vec<Triangle>,
    camera: PerspectiveCamera
}

fn to_hashmap<T>(a: Vec<(u32, T)>) -> HashMap<u32, T>{
    let mut map = HashMap::new();
    for (k, v) in a {
        map.insert(k, v);
    }
    map
}

impl AMDLLoader{
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self>{
        let mut f = File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf)?;
        return Self::from_bytes(buf);
    }
    pub fn from_bytes(data: Vec<u8>) -> Result<Self>{
        let scene = mdl::Scene::decode(data.as_slice()).ok_or(anyhow!("Unable to decode scene file"))?;
        return Self::from_scene(scene);
    }

    pub fn from_scene(scene: mdl::Scene) -> Result<Self>{
        let mut triangles: Vec<Triangle> = Vec::new();
        let focal_distance = 0.595877;
        let camera = PerspectiveCamera::from_euler(scene.camera.position.into(), scene.camera.rotation.into(), focal_distance);
        let faces = to_hashmap(scene.model.faces);
        let points = to_hashmap(scene.model.points);
        for (_, solid) in &scene.model.solids[1..]{
            let faces: Vec<_> = solid.faces.iter().map(|id|faces.get(id)).into_iter().flatten().collect();
            if faces.len() != 6 {
                return Err(anyhow!("Invalid face ID"));
            }
            for face in faces{
                //Counterclockwise
                let points: Vec<_> = face.points.iter().map(|id|points.get(id)).into_iter().flatten().collect();
                if points.len() != 4 {
                    return Err(anyhow!("Invalid face ID"));
                }
                let point_positions: Vec<Vec3> = points.iter().map(|point|(&point.position).into()).collect();
                let point0 = {let mut a = point_positions[0]; a[2] = -a[2]; a};
                let point1 = {let mut a = point_positions[1]; a[2] = -a[2]; a};
                let point2 = {let mut a = point_positions[2]; a[2] = -a[2]; a};
                let point3 = {let mut a = point_positions[3]; a[2] = -a[2]; a};
                let triangle1 = Triangle::new([point0, point2, point1], Vec3::from_single(0.5));
                let triangle2 = Triangle::new([point0, point3, point2], Vec3::from_single(0.5));
                triangles.push(triangle1);
                triangles.push(triangle2);
            }

        }
        Ok(Self{
            camera,
            triangles
        })
    }
}

impl Loader for AMDLLoader {
    type C = PerspectiveCamera;

    fn get_triangles(&self) -> &Vec<Triangle> {
        return &self.triangles
    }

    fn get_camera(&self) -> &Self::C {
        return &self.camera
    }
}

