use std::fmt::Display;

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
                println!("Fatal error: {}", message);
                println!();
                println!("Caused by:");

                let err = format!("{}", err);
                for line in err.lines() {
                    println!("    {}", line);
                }

                std::process::exit(-1);
            }
        }
    }
}
