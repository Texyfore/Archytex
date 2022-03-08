use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

mod defs;
mod fsutil;
mod repo;
mod require;

fn main() {
    let matches = cmd();
    let root = PathBuf::from(matches.value_of("in").unwrap());
    let (textures, props) = defs::read(&root);

    println!("Textures: {:?}", textures);
    println!("Props: {:?}", props);
}

fn cmd() -> ArgMatches {
    Command::new("kleng")
        .arg(Arg::new("in").required(true))
        .get_matches()
}
