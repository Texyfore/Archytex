mod defs;
mod report;
mod texture;

use clap::{ArgMatches, Command};

use crate::{defs::parse_defs, texture::enumerate_textures};

fn main() {
    let _matches = cmd();
    println!("{:?}", parse_defs("example/props/defs.json"));
    println!("{:?}", enumerate_textures("example"));
}

fn cmd() -> ArgMatches {
    Command::new("kleng").get_matches()
}
