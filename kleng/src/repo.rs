pub struct Repo {
    pub textures: Vec<Entry>,
    pub props: Vec<Entry>,
}

pub struct Entry {
    pub name: String,
    pub id: u32,
    pub public: Option<Public>,
}

pub struct Public {
    pub categories: Vec<String>,
}
