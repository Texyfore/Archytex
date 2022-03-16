use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assets {
    pub textures: HashMap<String, Texture>,
    pub props: HashMap<String, Prop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Texture {
    pub source: String,
    pub categories: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
    pub source: String,
    pub categories: Vec<String>,
    pub diffuse: HashMap<String, String>,
    pub emissive: HashMap<String, String>,
}
