use thiserror::Error;

#[derive(Error, Debug)]
#[error("Cannot decode corrupted buffer: {source}")]
pub struct DecodeError {
    #[source]
    pub(crate) source: Box<bincode::ErrorKind>,
}
