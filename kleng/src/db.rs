use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Db {
    pub textures: Vec<DbTexture>,
    pub props: Vec<DbProp>,
}

#[derive(Serialize, Deserialize)]
pub struct DbTexture {
    pub name: String,
    pub id: u32,
    pub public: bool,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct DbProp {
    pub name: String,
    pub id: u32,
    pub path: String,
}
