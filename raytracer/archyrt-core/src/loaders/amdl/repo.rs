use std::{collections::{hash_map::DefaultHasher, HashMap}, hash::{Hash, Hasher}, path::Path, fs::File};

use anyhow::Result;
use serde::{Serialize, Deserialize};

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
}

#[derive(Serialize, Deserialize)]
struct PropInfo {
    pub id: u32,
    pub url: String,
}

pub fn load_into(repo: &mut PropRepository, directory: &str) -> Result<()> {
    let propsjson = Path::new(directory).join("props.json");
    let propsjson = File::open(propsjson)?;
    let json: Vec<PropInfo> = serde_json::from_reader(propsjson)?;
    for prop in json {
        let path = Path::new(directory).join(prop.url);
        repo.insert(
            PropType::default(prop.id),
            AMDLLoader::from_path(path)?
        );
    }
    Ok(())
}
