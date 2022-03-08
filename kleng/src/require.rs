use std::error::Error;

use colored::Colorize;

pub trait Require<T> {
    fn require(self) -> T;
}

impl<T, E> Require<T> for Result<T, E>
where
    E: Error,
{
    fn require(self) -> T {
        match self {
            Ok(t) => t,
            Err(err) => {
                println!("{}: {}", "Error".bold().red(), err);
                if let Some(source) = err.source() {
                    println!("{}:", "Caused by".bold().red());
                    for line in format!("{}", source).lines() {
                        println!("    {}", line);
                    }
                }
                std::process::exit(-1);
            }
        }
    }
}
