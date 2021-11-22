use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use cgmath::Angle;
use std::path::Path;

use crate::cameras::perspective::PerspectiveCamera;
use crate::intersectables::triangle::Triangle;
use crate::matrix;

use crate::utilities::math::Vec3;

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
                    camera.right().into(),
                    camera.up().into(),
                    camera.forward().into()
                ),
            };
            Some(camera)
        }
        .ok_or(anyhow!("No camera found"))?;
        let triangles = if scene.models.len() <= 0 {
            None
        } else {
            let model = &scene.models[0];
            let triangles = model
                .triangles()
                .map_err(|_| anyhow!("Could not get triangles"))?;
            Some(
                triangles
                    .iter()
                    .map(|triangle| {
                        Triangle::new(
                            [
                                triangle[0].position.into(),
                                triangle[1].position.into(),
                                triangle[2].position.into(),
                            ],
                            Vec3::from_single(1.0),
                        )
                    })
                    .collect(),
            )
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
