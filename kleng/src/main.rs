mod compile;
mod db;
mod props;
mod report;
mod textures;

use clap::{Arg, ArgMatches, Command};
use compile::compile;
use props::enumerate_props;

use crate::{props::parse_defs, textures::enumerate_textures};

fn main() {
    let matches = cmd();
    let root = matches.value_of("in").unwrap();
    let textures = enumerate_textures(root);

    let props = {
        let defs = parse_defs(root);
        enumerate_props(root, defs, &textures)
    };

    compile(root, textures, props);
}

fn cmd() -> ArgMatches {
    Command::new("kleng")
        .arg(Arg::new("in").required(true))
        .get_matches()
}
