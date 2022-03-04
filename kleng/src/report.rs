use std::fmt::Display;

use colored::Colorize;

pub trait OrBail<T, E> {
    fn or_bail(self, message: &str) -> T;
}

impl<T, E> OrBail<T, E> for Result<T, E>
where
    E: Display,
{
    fn or_bail(self, message: &str) -> T {
        match self {
            Ok(ok) => ok,
            Err(err) => {
                println!("{}: {}", "Error".bold().red(), message);
                println!("{}:", "Caused by".bold().red());

                let err = format!("{}", err);
                for line in err.lines() {
                    println!("    {}", line);
                }

                std::process::exit(-1);
            }
        }
    }
}

pub fn bail(message: &str) {
    println!("{}: {}", "Error".bold().red(), message);
    std::process::exit(-1);
}

pub fn warn(message: &str) {
    println!("{}: {}", "Warning".bold().yellow(), message);
}
