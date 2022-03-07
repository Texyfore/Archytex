use std::collections::HashMap;

use asset::{BoundingBox, PropID};

#[derive(Default)]
pub struct PropInfoContainer {
    info: HashMap<PropID, PropInfo>,
}

impl PropInfoContainer {
    pub fn insert(&mut self, id: PropID, bounds: BoundingBox) {
        self.info.insert(id, PropInfo { bounds });
    }

    pub fn get(&self, id: PropID) -> Option<&PropInfo> {
        self.info.get(&id)
    }
}

pub struct PropInfo {
    pub bounds: BoundingBox,
}
