use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

mod defs;
mod fsutil;
mod indexed;
mod repo;
mod require;

fn main() {
    let matches = cmd();
    let root = PathBuf::from(matches.value_of("in").unwrap());

    let (textures, props) = defs::read(&root);
    let indexed = indexed::index(&root, textures, props);
    println!("{:#?}", indexed);
}

fn cmd() -> ArgMatches {
    Command::new("kleng")
        .arg(Arg::new("in").required(true))
        .get_matches()
}
