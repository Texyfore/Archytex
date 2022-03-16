pub mod amdl_textures;
use crate::intersectables::triangle::Triangle;
use crate::loaders::Loader;

use crate::renderers::path_tracer::Material;

use crate::utilities::math::{Vec2, Vec3, Matrix3x3};
use crate::{cameras::perspective::PerspectiveCamera, vector};
use anyhow::{anyhow, Result};
use asset::scene::{Scene, Point};
use cgmath::{Rotation, Matrix3, Matrix, SquareMatrix};

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use std::path::Path;

use self::amdl_textures::AMDLTextureType;

use super::amdl::repo::{PropRequest, PropType};


pub struct ASCNLoader {
    triangles: Vec<Triangle>,
    camera: PerspectiveCamera,
    prop_requests: Vec<PropRequest>
}
fn texcoord(position: Vec3, normal: Vec3) -> Vec2 {
    (if normal.x().abs() > normal.y().abs() {
        if normal.x().abs() > normal.z().abs() {
          vector!(position.z(), position.y())
        } else {
          vector!(position.x(), position.y())
        }
      } else if normal.y().abs() > normal.z().abs() {
        vector!(position.x(), position.z())
      } else {
          vector!(position.x(), position.y())
      }) / 4.0
}

impl ASCNLoader {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf)?;
        Self::from_bytes(&buf)
    }
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let scene = Scene::decode(data).ok_or_else(||anyhow!("Could not decode scene"))?;
        Self::from_scene(scene)
    }

    pub fn from_scene(scene: Scene) -> Result<Self> {
        let mut triangles: Vec<Triangle> = Vec::new();
        let focal_distance = 0.595877;
        let mut camera_pos: Vec3 = scene.camera.position.into();
        camera_pos.inner[2] = -camera_pos[2];
        let rotation: Vec2 = scene.camera.rotation.into();
        let mut camera = PerspectiveCamera::from_euler(
            camera_pos,
            vector![rotation.x(), rotation.y(), 0.0] / 180.0 * std::f64::consts::PI,
            focal_distance,
        );
        camera.matrix = camera.matrix.transpose();
        for solid in &scene.world.solids {
            for face in &solid.faces {
                if face.texture.0 == 0 {
                    continue;
                }
                //Counterclockwise
                let points: Vec<&Point> = face
                    .indices
                    .iter()
                    .map(|id| &solid.points[(*id) as usize])
                    .collect();
                let point_positions: Vec<Vec3> = points
                    .iter()
                    .map(|point| (point.position).into())
                    .map(|point: Vec3| point/100.0)
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
                    AMDLTextureType::diffuse(face.texture.0),
                    Material::Diffuse,
                );
                let triangle2 = Triangle::new(
                    [point0, point3, point2],
                    [uv0, uv3, uv2],
                    AMDLTextureType::diffuse(face.texture.0),
                    Material::Diffuse,
                );
                triangles.push(triangle1);
                triangles.push(triangle2);
            }
        }
        let prop_requests: Vec<PropRequest> = scene.world.props.iter().map(|prop|{
            let mut pos: Vec3 = prop.position.into();
            pos.inner[2] = -pos.inner[2];
            pos = pos/100.0;
            let matrix: Matrix3<f32> = prop.rotation.into();
            let mut matrix = matrix.transpose();
            matrix.z = -matrix.z;
            let inverse_matrix = matrix.invert().unwrap();
            

            PropRequest{
                prop: PropType::default(prop.asset.0),
                position: pos,
                matrix: matrix.into(),
                inverse_matrix: inverse_matrix.into()
            }
        }).collect();
        Ok(Self { camera, triangles, prop_requests })
    }
    pub fn get_prop_requests(&self) -> &Vec<PropRequest>{
        &self.prop_requests
    }
}

impl Loader for ASCNLoader {
    type C = PerspectiveCamera;

    fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn get_camera(&self) -> &Self::C {
        &self.camera
    }
}
