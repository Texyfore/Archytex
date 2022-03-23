mod amdl;
mod input;
mod repo;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use amdl::gltf_to_amdl;
use clap::{Arg, ArgMatches, Command};
use image::{imageops::FilterType, ImageFormat};
use input::Assets;

fn main() {
    let matches = cmd();
    let root = PathBuf::from(matches.value_of("in").unwrap());

    let assets: Assets = {
        let file = File::open(root.join("assets.json")).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    fs::create_dir_all(root.join("out/public/textures")).unwrap();
    fs::create_dir_all(root.join("out/public/props")).unwrap();
    fs::create_dir_all(root.join("out/raytracer/textures")).unwrap();
    fs::create_dir_all(root.join("out/raytracer/props")).unwrap();

    for (name, texture) in &assets.textures {
        let diffuse = {
            let file = File::open(root.join("textures").join(&texture.diffuse)).unwrap();
            let reader = BufReader::new(file);
            image::load(reader, ImageFormat::Png).unwrap()
        };

        let emissive = texture.emissive.as_ref().map(|emissive| {
            let file = File::open(root.join("textures").join(emissive)).unwrap();
            let reader = BufReader::new(file);
            image::load(reader, ImageFormat::Png).unwrap()
        });

        let diffuse_small = diffuse.resize_exact(256, 256, FilterType::CatmullRom);

        diffuse
            .save(root.join(format!("out/raytracer/textures/{}.png", name)))
            .unwrap();

        diffuse_small
            .save(root.join(format!("out/public/textures/{}.png", name)))
            .unwrap();

        if let Some(emissive) = emissive {
            emissive
                .save(root.join(format!(
                    "out/raytracer/textures/{}.png",
                    texture.emissive.as_ref().unwrap()
                )))
                .unwrap();
        }
    }

    let indexed_textures: HashMap<String, Indexed<input::Texture>> = assets
        .textures
        .into_iter()
        .enumerate()
        .map(|(i, (name, texture))| (name, Indexed::new(i as u32 + 2, texture)))
        .collect();

    for (name, prop) in &assets.props {
        let prop = gltf_to_amdl(&root, prop, &indexed_textures);
        let buf = prop.encode().unwrap();
        fs::write(root.join(format!("out/public/props/{}.amdl", name)), &buf).unwrap();
        fs::write(
            root.join(format!("out/raytracer/props/{}.amdl", name)),
            &buf,
        )
        .unwrap();
    }

    let indexed_props: HashMap<String, Indexed<input::Prop>> = assets
        .props
        .into_iter()
        .enumerate()
        .map(|(i, (name, prop))| (name, Indexed::new(i as u32, prop)))
        .collect();

    {
        let props = indexed_props
            .into_iter()
            .map(|(name, prop)| repo::Prop {
                id: prop.id,
                name,
                categories: prop.value.categories,
                dependencies: prop.value.textures.into_values().collect(),
            })
            .collect();

        let textures = indexed_textures
            .into_iter()
            .map(|(name, texture)| repo::Texture {
                id: texture.id,
                name,
                categories: texture.value.categories,
                emissive: texture.value.emissive,
            })
            .collect();

        let repo = repo::Repo { textures, props };
        let json = serde_json::to_string_pretty(&repo).unwrap();
        fs::write(root.join("out/repo.json"), json).unwrap();
    }
}

fn cmd() -> ArgMatches {
    Command::new("kleng")
        .arg(Arg::new("in").required(true))
        .get_matches()
}

pub struct Indexed<T> {
    pub id: u32,
    pub value: T,
}

impl<T> Indexed<T> {
    fn new(id: u32, value: T) -> Self {
        Self { id, value }
    }
}
