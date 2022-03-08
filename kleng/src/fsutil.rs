use std::path::{Path, PathBuf};

pub struct CanonPath {
    inner: PathBuf,
}

impl CanonPath {
    pub fn new<P>(path: P) -> Option<Self>
    where
        P: AsRef<Path>,
    {
        let buf = path.as_ref().to_owned();
        buf.canonicalize().ok().map(|inner| Self { inner })
    }
}

impl AsRef<Path> for CanonPath {
    fn as_ref(&self) -> &Path {
        self.inner.as_ref()
    }
}
