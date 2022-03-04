mod compile;
mod props;
mod report;
mod textures;

use clap::{Arg, ArgMatches, Command};
use compile::compile;

use crate::{props::parse_defs, textures::enumerate_textures};

fn main() {
    let matches = cmd();
    let root = matches.value_of("in").unwrap();
    let textures = enumerate_textures(root);
    let prop_defs = parse_defs(root);
    compile(root, &textures, &prop_defs);
}

fn cmd() -> ArgMatches {
    Command::new("kleng")
        .arg(Arg::new("in").required(true))
        .get_matches()
}
