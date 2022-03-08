use std::{
    error::Error,
    path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug)]
pub struct CanonPath {
    inner: PathBuf,
}

impl CanonPath {
    pub fn new<P>(path: P) -> Result<Self, CanonError>
    where
        P: AsRef<Path>,
    {
        let buf = path.as_ref().to_owned();
        let inner = match buf.canonicalize() {
            Ok(ok) => ok,
            Err(err) => {
                return Err(CanonError {
                    path: buf,
                    source: Box::new(err),
                })
            }
        };

        Ok(Self { inner })
    }
}

impl AsRef<Path> for CanonPath {
    fn as_ref(&self) -> &Path {
        self.inner.as_ref()
    }
}

#[derive(Debug, Error)]
#[error("Failed to canonicalize path `{path}`")]
pub struct CanonError {
    path: PathBuf,

    #[source]
    source: Box<dyn Error>,
}
