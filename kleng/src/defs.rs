use std::{collections::HashMap, fs::File, path::Path};

use serde::{Deserialize, Serialize};

use crate::report::OrBail;

#[derive(Serialize, Deserialize, Debug)]
pub struct PropDef {
    pub name: String,
    pub source: String,
    pub textures: HashMap<String, String>,
}

pub fn parse_defs<P>(path: P) -> Vec<PropDef>
where
    P: AsRef<Path>,
{
    let file = File::open(path).or_bail("couldn't open defs.json");
    serde_json::from_reader(&file).or_bail("couldn't parse defs.json")
}
