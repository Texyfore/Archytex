use std::{path::Path, fs::File, io::Read};

use crate::{intersectables::triangle::Triangle, renderers::path_tracer::Material};
use anyhow::{anyhow, Result};
use asset::Prop;

use super::ascn::amdl_textures::AMDLTextureType;
pub mod repo;

pub struct AMDLLoader{
    pub triangles: Vec<Triangle>
}

impl AMDLLoader{
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut f = File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf)?;
        Self::from_bytes(&buf)
    }
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let scene = Prop::decode(data).ok_or_else(||anyhow!("Could not decode scene"))?;
        Self::from_prop(scene)
    }

    pub fn from_prop(scene: Prop) -> Result<Self> {
        let mut triangles = Vec::new();
        for mesh in scene.meshes{
            let texture = AMDLTextureType::diffuse(mesh.texture.0);
            for triangle in mesh.triangles{
                let triangle: Vec<&asset::PropVertex> = triangle.iter().map(|index|&mesh.vertices[(*index) as usize]).collect();
                let v1 = triangle[0];
                let v2 = triangle[1];
                let v3 = triangle[2];
                let triangle = Triangle::new(
                    [
                        v1.position.into(),
                        v2.position.into(),
                        v3.position.into(),
                    ],
                    [
                        v1.texcoord.into(),
                        v2.texcoord.into(),
                        v3.texcoord.into(),
                    ],
                    texture,
                    Material::Diffuse
                );
                triangles.push(triangle);
            }
        }
        Ok(Self{
            triangles
        })
    }

}