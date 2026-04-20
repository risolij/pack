use std::fs;
use std::path::PathBuf;
use crate::gear::{Gear, Ditchable};

pub trait Ditcher<G>
where
    G: Ditchable
{
    fn ditch(&self, path: &PathBuf, name: &str) -> bool;
}

pub struct GearDitcher;

impl GearDitcher {
    pub fn new() -> Self {
        Self {}
    }
}

impl Ditcher<Gear> for GearDitcher {
    fn ditch(&self, path: &PathBuf, name: &str) -> bool {
        let removal = fs::remove_file(path.join(name));

        let response = if removal.is_ok() { true } else { false };

        response
    }
}
