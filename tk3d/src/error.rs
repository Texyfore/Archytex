use thiserror::Error;

#[derive(Error, Debug)]
#[error("Cannot decode corrupted buffer: {source}")]
pub struct DecodeError {
    #[source]
    pub(crate) source: Box<bincode::ErrorKind>,
}

#[derive(Error, Debug)]
pub enum MeshGenError {
    #[error("Cannot generate mesh: Bad point reference ({0})")]
    BadPointRef(u32),
    
    #[error("Cannot generate mesh: Bad face reference ({0})")]
    BadFaceRef(u32),
}
