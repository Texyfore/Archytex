use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub textures: Vec<Texture>,
    pub props: Vec<Prop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Texture {
    pub name: String,
    pub id: u32,
    pub public: Option<Public>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
    pub name: String,
    pub id: u32,
    pub dependencies: Vec<String>,
    pub public: Option<Public>,
    pub emissive: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Public {
    pub categories: Vec<String>,
}
