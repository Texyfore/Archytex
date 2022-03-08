use clap::{Arg, ArgMatches, Command};

mod fsutil;
mod require;

fn main() {
    let _matches = cmd();
}

fn cmd() -> ArgMatches {
    Command::new("kleng")
        .arg(Arg::new("in").required(true))
        .get_matches()
}
