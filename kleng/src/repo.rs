use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub textures: Vec<Texture>,
    pub props: Vec<Prop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Texture {
    pub id: u32,
    pub name: String,
    pub categories: Vec<String>,
    pub emissive: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
    pub id: u32,
    pub name: String,
    pub categories: Vec<String>,
    pub dependencies: Vec<String>,
}
