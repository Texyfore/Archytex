use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use cgmath::Angle;
use std::convert::identity;
use std::path::Path;

use crate::cameras::perspective::PerspectiveCamera;
use crate::intersectables::triangle::Triangle;
use crate::matrix;

use crate::utilities::math::Vec3;
use crate::utilities::math::Vector;

use super::Loader;

pub struct GltfLoader {
    camera: PerspectiveCamera,
    triangles: Vec<Triangle>,
}

impl GltfLoader {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let scene = easy_gltf::load(path).map_err(|_| anyhow!("Could not open glTF file"))?;
        if scene.len() <= 0 {
            bail!("Could not find first scene");
        }
        let scene = &scene[0];
        let camera = if scene.cameras.len() <= 0 {
            None
        } else {
            let camera = &scene.cameras[0];
            let focal_distance = 1.0 / ((camera.fov / 2.0).tan() * 2.0) as f64;
            let camera = PerspectiveCamera {
                position: camera.position().into(),
                focal_distance: focal_distance,
                matrix: matrix!(
                    Vector::from(camera.right()),
                    Vector::from(camera.up()),
                    -Vector::from(camera.forward())
                ),
            };
            Some(camera)
        }
        .ok_or(anyhow!("No camera found"))?;
        let triangles: Vec<Triangle> = if scene.models.len() <= 0 {
            None
        } else {
            let model: Vec<Triangle> = (&scene)
                .models
                .iter()
                .map(|model| {
                    let triangles = model.triangles();
                    let triangles = if let Ok(triangles) = triangles {
                        triangles
                    } else {
                        return None;
                    };
                    Some(
                        triangles
                            .iter()
                            .map(|triangle| {
                                let mut t = Triangle::new(
                                    [
                                        triangle[0].position.into(),
                                        triangle[1].position.into(),
                                        triangle[2].position.into(),
                                    ],
                                    Vec3::from_single(1.0),
                                );
                                t.normal = triangle[0].normal.into();
                                t.normal += triangle[1].normal.into();
                                t.normal += triangle[2].normal.into();
                                t.normal /= 3.0;
                                t.normal = t.normal.normalized();
                                t
                            })
                            .collect::<Vec<Triangle>>(),
                    )
                })
                .filter_map(identity)
                .flatten()
                .collect();
            Some(model)
        }
        .ok_or(anyhow!("Could not get model"))?;
        Ok(GltfLoader { camera, triangles })
    }
}

impl Loader for GltfLoader {
    type C = PerspectiveCamera;

    fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }

    fn get_camera(&self) -> &PerspectiveCamera {
        &self.camera
    }
}
