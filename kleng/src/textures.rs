use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    path::{Path, PathBuf},
};

use crate::report::OrBail;

#[derive(Debug)]
pub struct Texture {
    pub id: u32,
    pub path: PathBuf,
    pub public: bool,
}

pub fn enumerate_textures(root: &str) -> HashMap<String, Texture> {
    let mut next_id = 2;
    let mut textures = HashMap::new();

    traverse(
        format!("{}/textures", root),
        &mut next_id,
        &mut textures,
        true,
    );

    traverse(
        format!("{}/props", root),
        &mut next_id,
        &mut textures,
        false,
    );

    textures
}

fn traverse<P>(path: P, next_id: &mut u32, map: &mut HashMap<String, Texture>, public: bool)
where
    P: AsRef<Path> + Display,
{
    let dir = fs::read_dir(&path).or_bail(&format!("couldn't open directory `{}`", path));

    for entry in dir {
        let entry = entry.or_bail(&format!("couldn't access file in `{}`", path));
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.split('.').next());

        let ext = path.extension().and_then(|ext| ext.to_str());

        match (name, ext) {
            (Some(name), Some("png" | "jpg")) => {
                map.insert(
                    name.to_owned(),
                    Texture {
                        id: *next_id,
                        path,
                        public,
                    },
                );
                *next_id += 1;
            }
            _ => (),
        }
    }
}
