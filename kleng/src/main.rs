use std::{fs, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use require::Require;

mod compiler;
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

    compiler::save(&root, &indexed);

    {
        let repo = repo::create(indexed);
        let json = serde_json::to_string_pretty(&repo).require();
        fs::write(root.join("out/repo.json"), json).require();
    }
}

fn cmd() -> ArgMatches {
    Command::new("kleng")
        .arg(Arg::new("in").required(true))
        .get_matches()
}
