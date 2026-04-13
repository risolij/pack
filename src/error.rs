use thiserror::Error;

#[derive(Debug, Error)]
pub enum PackError {
    #[error("Gear pack issue")]
    Io(#[from] std::io::Error),

    #[error("Gear Invalid Path")]
    GearInvalidPath,

    #[error("Gear Environment error")]
    Environment(#[from] std::env::VarError)
}
