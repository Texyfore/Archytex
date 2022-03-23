use std::collections::HashSet;

use asset::scene::Scene;

pub struct Ascn {
    scene: Scene,
}

impl Ascn {
    pub fn new(buf: &[u8]) -> Option<Self> {
        Scene::decode(buf).map(|scene| Self { scene })
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn textures(&self) -> Vec<u32> {
        let mut set = HashSet::new();

        for solid in &self.scene.world.solids {
            for face in &solid.faces {
                set.insert(face.texture.0);
            }
        }

        set.into_iter().collect()
    }

    pub fn props(&self) -> Vec<u32> {
        let mut set = HashSet::new();

        for prop in &self.scene.world.props {
            set.insert(prop.asset.0);
        }

        set.into_iter().collect()
    }
}
