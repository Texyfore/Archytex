use std::{collections::HashMap, path::Path};

use thiserror::Error;

use crate::{defs::PropDef, fsutil::CanonPath, require::Require};

#[derive(Debug)]
pub struct Indexed {
    pub textures: Vec<Texture>,
    pub props: Vec<Prop>,
}

#[derive(Debug)]
pub struct Texture {
    pub name: String,
    pub id: u32,
    pub path: CanonPath,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Prop {
    pub name: String,
    pub id: u32,
    pub path: CanonPath,
    pub categories: Option<Vec<String>>,
    pub textures: Option<HashMap<String, String>>,
    pub dependencies: Vec<u32>,
}

pub fn index(
    root: &Path,
    textures: HashMap<String, Vec<String>>,
    props: HashMap<String, PropDef>,
) -> Indexed {
    let mut prop_deps: HashMap<String, Vec<u32>> = HashMap::new();

    let textures = {
        let mut next_id = 2;
        let mut entries = Vec::new();

        for (name, categories) in textures {
            if categories.is_empty() {
                Result::<(), _>::Err(NoCategoriesError(&name)).require();
            }

            let path = root.join(format!("textures/{}.png", name));
            let path = CanonPath::new(path).require();

            entries.push(Texture {
                name,
                id: next_id,
                path,
                categories: Some(categories),
            });

            next_id += 1;
        }

        for (name, prop) in &props {
            let textures = prop
                .textures
                .as_ref()
                .map(|textures| textures.values().cloned().collect())
                .unwrap_or_else(|| vec![name.clone()]);

            for texture in textures {
                let path = root.join(format!("props/{}.png", texture));
                let path = CanonPath::new(path).require();

                entries.push(Texture {
                    name: texture,
                    id: next_id,
                    path,
                    categories: None,
                });

                prop_deps.entry(name.clone()).or_default().push(next_id);
                next_id += 1;
            }
        }

        entries
    };

    let props = {
        let mut next_id = 0;
        let mut entries = Vec::new();

        #[allow(clippy::explicit_counter_loop)]
        for (name, prop) in props {
            if prop.categories.is_empty() {
                Result::<(), _>::Err(NoCategoriesError(&name)).require();
            }

            let path = root.join(format!("props/{}.gltf", name));
            let path = CanonPath::new(path).require();
            let dependencies = prop_deps.remove(&name).unwrap();

            entries.push(Prop {
                name,
                id: next_id,
                path,
                categories: Some(prop.categories),
                textures: prop.textures,
                dependencies,
            });

            next_id += 1;
        }

        entries
    };

    Indexed { textures, props }
}

#[derive(Debug, Error)]
#[error("No categories provided for `{0}`")]
struct NoCategoriesError<'a>(&'a str);
