use std::{collections::HashMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::{fsutil::CanonPath, require::Require};

#[derive(Serialize, Deserialize, Debug)]
pub struct TextureDef {
    pub name: String,
    pub source: String,
    pub categories: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropDef {
    pub name: String,
    pub source: String,
    pub textures: HashMap<String, String>,
    pub categories: Vec<String>,
}

pub fn read(root: &Path) -> (Vec<TextureDef>, Vec<PropDef>) {
    let textures = {
        let path = CanonPath::new(root.join("textures/defs.json")).require();
        let file = File::open(path).require();
        serde_json::from_reader(file).require()
    };

    let props = {
        let path = CanonPath::new(root.join("props/defs.json")).require();
        let file = File::open(path).require();
        serde_json::from_reader(file).require()
    };

    (textures, props)
}
