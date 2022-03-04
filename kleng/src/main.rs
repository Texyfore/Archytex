mod error;

use clap::{ArgMatches, Command};

fn main() {
    let _matches = cmd();
}

fn cmd() -> ArgMatches {
    Command::new("kleng").get_matches()
}
