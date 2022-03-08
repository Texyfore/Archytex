use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub textures: Vec<Entry>,
    pub props: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub name: String,
    pub id: u32,
    pub public: Option<Public>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Public {
    pub categories: Vec<String>,
}
