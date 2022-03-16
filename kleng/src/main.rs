mod input;
mod repo;

use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use input::Assets;

fn main() {
    let matches = cmd();
    let root = PathBuf::from(matches.value_of("in").unwrap());

    let assets: Assets = {
        let file = File::open(root.join("assets.json")).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    let texture_indices: HashMap<String, u32> = assets
        .textures
        .keys()
        .cloned()
        .enumerate()
        .map(|(i, name)| (name, i as u32 + 2))
        .collect();

    let prop_indices: HashMap<String, u32> = assets
        .props
        .keys()
        .cloned()
        .enumerate()
        .map(|(i, name)| (name, i as u32))
        .collect();

    println!("{:#?}", assets);
}

fn cmd() -> ArgMatches {
    Command::new("kleng")
        .arg(Arg::new("in").required(true))
        .get_matches()
}
