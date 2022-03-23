use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub textures: HashMap<String, Texture>,
    pub props: HashMap<String, Prop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Texture {
    pub diffuse: String,
    pub emissive: Option<String>,
    pub categories: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
    pub source: String,
    pub categories: Vec<String>,
    pub textures: HashMap<String, String>,
}
