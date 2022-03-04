use std::{collections::HashMap, fs};

use crate::{props::PropDef, report::OrBail, textures::Texture};

pub fn compile(root: &str, textures: &HashMap<String, Texture>, defs: &[PropDef]) {
    mkdir(root, "out/textures/editor");
    mkdir(root, "out/textures/raytracer");
    mkdir(root, "out/props");

    for (name, texture) in textures {
        let image = {
            let buf = fs::read(&texture.path)
                .or_bail(&format!("couldn't read file `{:?}`", &texture.path));

            image::load_from_memory(&buf)
                .or_bail(&format!("couldn't parse texture `{:?}`", &texture.path))
        };

        let small = image.resize_exact(256, 256, image::imageops::FilterType::CatmullRom);

        image
            .save_with_format(
                format!("{}/out/textures/raytracer/{}.png", root, name),
                image::ImageFormat::Png,
            )
            .or_bail(&format!("failed to save texture `{:?}`", &texture.path));

        small
            .save_with_format(
                format!("{}/out/textures/editor/{}.png", root, name),
                image::ImageFormat::Png,
            )
            .or_bail(&format!("failed to save texture `{:?}`", &texture.path));
    }
}

fn mkdir(root: &str, path: &str) {
    let path = format!("{}/{}", root, path);
    OrBail::or_bail(
        fs::create_dir_all(&path),
        &format!("couldn't create directory `{}`", path),
    );
}
