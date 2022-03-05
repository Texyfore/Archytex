use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Db {
    pub textures: Vec<DbTexture>,
    pub props: Vec<DbProp>,
}

#[derive(Serialize, Deserialize)]
pub struct DbTexture {
    pub id: u32,
    pub name: String,
    pub public: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DbProp {
    pub id: u32,
    pub name: String,
}
