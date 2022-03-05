use std::{collections::HashMap, fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    report::{bail, OrBail},
    textures::Texture,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct PropDef {
    pub name: String,
    pub source: String,
    pub textures: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Prop {
    pub name: String,
    pub id: u32,
    pub source: PathBuf,
    pub textures: HashMap<String, u32>,
}

pub fn parse_defs(root: &str) -> Vec<PropDef> {
    let path = format!("{}/props/defs.json", root);
    let file = File::open(path).or_bail("couldn't open defs.json");
    serde_json::from_reader(&file).or_bail("couldn't parse defs.json")
}

pub fn enumerate_props(
    root: &str,
    defs: Vec<PropDef>,
    textures: &HashMap<String, Texture>,
) -> Vec<Prop> {
    let mut next_id = 1;
    let mut props = Vec::new();

    for def in defs {
        let textures = def
            .textures
            .into_iter()
            .map(|(mesh, texture)| {
                let id = textures
                    .get(&texture)
                    .unwrap_or_else(|| bail(&format!("no texture named `{}`", texture)))
                    .id;

                (mesh, id)
            })
            .collect();

        let prop = Prop {
            name: def.name,
            id: next_id,
            source: format!("{}/props/{}", root, def.source).into(),
            textures,
        };

        props.push(prop);
        next_id += 1;
    }

    props
}
