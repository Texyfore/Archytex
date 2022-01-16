use thiserror::Error;

#[derive(Error, Debug)]
#[error("Cannot decode corrupted buffer: {0}")]
pub struct DecodeError(#[from] Box<bincode::ErrorKind>);

#[derive(Error, Debug)]
#[error("Cannot generate mesh from corrupted model")]
pub struct MeshGenError;
