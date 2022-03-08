use serde::{Deserialize, Serialize};

use crate::indexed::{self, Indexed};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub textures: Vec<Texture>,
    pub props: Vec<Prop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Texture {
    pub name: String,
    pub id: u32,
    pub public: Option<Public>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prop {
    pub name: String,
    pub id: u32,
    pub dependencies: Vec<u32>,
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
            .map(|texture| texture.into())
            .collect(),

        props: indexed.props.into_iter().map(|prop| prop.into()).collect(),
    }
}

impl From<indexed::Texture> for Texture {
    fn from(texture: indexed::Texture) -> Self {
        Self {
            name: texture.name,
            id: texture.id,
            public: texture.categories.map(|categories| Public { categories }),
        }
    }
}

impl From<indexed::Prop> for Prop {
    fn from(prop: indexed::Prop) -> Self {
        Self {
            name: prop.name,
            id: prop.id,
            dependencies: prop.dependencies,
            public: prop.categories.map(|categories| Public { categories }),
        }
    }
}
