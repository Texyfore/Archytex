use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TextureID(pub u32);

#[derive(Serialize, Deserialize)]
pub struct PropID(pub u32);
