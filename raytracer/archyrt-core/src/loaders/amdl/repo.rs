use std::{collections::{hash_map::DefaultHasher, HashMap}, hash::{Hash, Hasher}, path::Path, fs::File};

use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

use crate::{intersectables::{bvh::{self, BVH}, apply_matrix::ApplyMatrix, transform::Transform}, utilities::math::{Vec3, Matrix3x3}, textures::texture_repo::TextureRepository};

use super::AMDLLoader;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct PropID(u64);

#[derive(Hash, Copy, Clone)]
pub enum PropType{
    Default(u32)
}

impl PropType{
    pub fn default(id: u32) -> PropID{
        PropID::new(&PropType::Default(id))
    }
}

impl PropID {
    pub fn new<T: Hash>(val: &T) -> Self {
        let mut hasher = DefaultHasher::new();
        val.hash(&mut hasher);
        Self(hasher.finish())
    }
}

impl Default for PropID {
    fn default() -> Self {
        Self::new(&0u32)
    }
}

pub struct PropRepository {
    pub objects: HashMap<PropID, AMDLLoader>,
}

impl PropRepository {
    pub fn new() -> Self {
        let t = HashMap::new();
        Self { objects: t }
    }
    pub fn get(&self, id: PropID) -> Option<&AMDLLoader> {
        let object = self.objects.get(&id)?;
        Some(object)
    }
    pub fn insert(&mut self, id: PropID, object: AMDLLoader) {
        self.objects.insert(id, object);
    }
    pub fn fulfill(&self, req: &PropRequest) -> Result<Transform<ApplyMatrix<BVH>>>{
        let object = self.get(req.prop).ok_or(anyhow!("Invalid prop id"))?;
        let object = &object.triangles;
        let object = BVH::from_triangles(object).ok_or(anyhow!("Invalid geometry"))?;
        let object = ApplyMatrix{
            inner: object, 
            matrix: req.matrix,
            inverse_matrix: req.inverse_matrix
        };
        let object = Transform{
            inner: object,
            transformation: req.position
        };
        Ok(object)
    }
    pub fn fulfill_all(&self, requests: &[PropRequest]) -> Result<Vec<Transform<ApplyMatrix<BVH>>>>{
        let mut output = Vec::with_capacity(requests.len());
        for req in requests{
            let prop = self.fulfill(req)?;
            output.push(prop);
        }
        Ok(output)
    }
}

#[derive(Serialize, Deserialize)]
struct PropInfo {
    pub id: u32,
    pub name: String
}

#[derive(Serialize, Deserialize)]
struct Repo{
    pub props: Vec<PropInfo>
}

pub fn load_into(repo: &mut PropRepository, textures: &TextureRepository, directory: &str) -> Result<()> {
    let propsjson = Path::new(directory).join("repo.json");
    let propsjson = File::open(propsjson)?;
    let json: Repo = serde_json::from_reader(propsjson)?;
    for prop in json.props {
        let path = Path::new(directory).join("props").join(prop.name).with_extension("amdl");
        repo.insert(
            PropType::default(prop.id),
            AMDLLoader::from_path(path, textures)?
        );
    }
    Ok(())
}


#[derive(Clone)]
pub struct PropRequest{
    pub prop: PropID,
    pub position: Vec3,
    pub matrix: Matrix3x3,
    pub inverse_matrix: Matrix3x3,
}