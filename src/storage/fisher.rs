use std::fs::{read, read_dir};
use std::path::PathBuf;

use crate::error::PackError;
use crate::gear::Gear;
use crate::cli::Extension;

pub trait Fisher<G> {
    fn fish(&self, path: &PathBuf, name: &str) -> Option<G>;
    fn dump(&self, path: &PathBuf) -> Result<Vec<G>, PackError>;
}

pub struct GearFisher;

impl GearFisher {
    pub fn new() -> Self {
        Self { }
    }
}
impl Fisher<Gear> for GearFisher {
    fn fish(&self, path: &PathBuf, name: &str) -> Option<Gear> {
        let path = path.join(name);
        let extension = Extension::Rs;
        Gear::from_file(name.to_string(), path.to_owned(), extension).ok()
    }

    fn dump(&self, path: &PathBuf) -> Result<Vec<Gear>, PackError> {
        if !path.exists() {
            return Err(PackError::PackNotFound);
        }

        let result: Result<Vec<_>, PackError> = read_dir(path)?
            .map(|entry| {
                let path = entry
                    .map_err(|_| PackError::GearNotFound)?
                    .path();

                let file_name = path
                    .file_name()
                    .ok_or(PackError::GearNotFound)?
                    .to_string_lossy()
                    .to_string();

                let contents = read(&path)?;

                Ok(Gear::new(file_name.to_string(), contents, Extension::Rs))
            })
            .collect();

        result
    }
}
