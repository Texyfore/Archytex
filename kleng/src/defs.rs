use std::{collections::HashMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::{fsutil::CanonPath, require::Require};

#[derive(Serialize, Deserialize, Debug)]
pub struct PropDef {
    pub categories: Vec<String>,
    pub textures: Option<HashMap<String, String>>,
}

pub fn read(root: &Path) -> (HashMap<String, Vec<String>>, HashMap<String, PropDef>) {
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
