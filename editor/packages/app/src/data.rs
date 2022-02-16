use std::collections::HashMap;

use asset::{BoundingBox, Prop, PropID};

#[derive(Default)]
pub struct PropInfoContainer {
    info: HashMap<PropID, PropInfo>,
}

impl PropInfoContainer {
    pub fn insert(&mut self, id: PropID, prop: &Prop) {
        self.info.insert(
            id,
            PropInfo {
                bounds: prop.bounds,
            },
        );
    }

    pub fn get(&self, id: PropID) -> Option<&PropInfo> {
        self.info.get(&id)
    }
}

pub struct PropInfo {
    pub bounds: BoundingBox,
}
