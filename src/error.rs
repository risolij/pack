use thiserror::Error;
use std::io::{self, ErrorKind};

#[derive(Debug, Error)]
pub enum PackError {
    #[error("Gear already exists in pack")]
    GearAlreadyExists,

    #[error("Pack not found")]
    PackNotFound,

    #[error("Gear not found in pack")]
    GearNotFound,

    #[error("Proto Gear not convertible to Gear")]
    ProtoGearConversion,

    #[error("Pack System unknown")]
    PackSystemUnkown,

    #[error("Unexpected error occurred: {0}")]
    Io(#[source] io::Error),

    #[error("Failed to load pack: {0}")]
    Environment(#[from] std::env::VarError)
}

impl From<io::Error> for PackError {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            ErrorKind::AlreadyExists => PackError::GearAlreadyExists,
            ErrorKind::NotFound => PackError::GearNotFound,
            _ => PackError::Io(e)

        }
    }
}
