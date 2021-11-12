use std::path::Path;

use anyhow::Result;
use anyhow::bail;

use crate::cameras::perspective::PerspectiveCamera;

pub struct GltfLoader {
    camera: Option<PerspectiveCamera>
}

impl GltfLoader {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let scene = easy_gltf::load(path)?;
        if scene.len() <= 0 {
            bail!("Could not find first scene");
        }
        let scene = scene[0];
    }
}

//impl Loader for GltfLoader {}
