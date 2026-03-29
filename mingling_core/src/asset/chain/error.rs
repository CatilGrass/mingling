use crate::AnyOutput;

#[derive(thiserror::Error, Debug)]
pub enum ChainProcessError {
    #[error("Other error: {0}")]
    Other(String),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Broken chain")]
    Broken(AnyOutput),
}
