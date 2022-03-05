use std::{collections::HashMap, fs};

use crate::{
    db::{Db, DbProp, DbTexture},
    props::Prop,
    report::OrBail,
    textures::Texture,
};

pub fn compile(root: &str, textures: HashMap<String, Texture>, props: Vec<Prop>) {
    mkdir(root, "out/textures/editor");
    mkdir(root, "out/textures/raytracer");
    mkdir(root, "out/props");

    let mut db = Db {
        textures: Vec::new(),
        props: Vec::new(),
    };

    for (name, texture) in textures {
        db.textures.push(DbTexture {
            id: texture.id,
            name: name.to_owned(),
            public: texture.public,
        });

        save_images(root, &name, &texture);
    }

    for prop in props {
        db.props.push(DbProp {
            id: prop.id,
            name: prop.name,
        });
    }

    fs::write(
        format!("{}/out/db.json", root),
        serde_json::to_string_pretty(&db).or_bail("failed to serialize `db.json`"),
    )
    .or_bail("failed to save `db.json`");
}

fn mkdir(root: &str, path: &str) {
    let path = format!("{}/{}", root, path);
    OrBail::or_bail(
        fs::create_dir_all(&path),
        &format!("couldn't create directory `{}`", path),
    );
}

fn save_images(root: &str, name: &str, texture: &Texture) {
    let large = {
        let buf =
            fs::read(&texture.path).or_bail(&format!("couldn't read file `{:?}`", &texture.path));

        image::load_from_memory(&buf).or_bail(&format!("couldn't parse texture `{}`", name))
    };

    let small = large.resize_exact(256, 256, image::imageops::FilterType::CatmullRom);

    large
        .save_with_format(
            format!("{}/out/textures/raytracer/{}.png", root, name),
            image::ImageFormat::Png,
        )
        .or_bail(&format!("failed to save texture `{}`", name));

    small
        .save_with_format(
            format!("{}/out/textures/editor/{}.png", root, name),
            image::ImageFormat::Png,
        )
        .or_bail(&format!("failed to save texture `{}`", name));
}
