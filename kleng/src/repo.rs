use serde::{Deserialize, Serialize};

use crate::indexed::{self, Indexed};

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

pub fn create(indexed: Indexed) -> Repo {
    Repo {
        textures: indexed
            .textures
            .into_iter()
            .map(|entry| entry.into())
            .collect(),

        props: indexed
            .props
            .into_iter()
            .map(|prop| prop.entry.into())
            .collect(),
    }
}

impl From<indexed::Entry> for Entry {
    fn from(entry: indexed::Entry) -> Self {
        Self {
            name: entry.name,
            id: entry.id,
            public: entry.categories.map(|categories| Public { categories }),
        }
    }
}
