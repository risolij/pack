use std::path::PathBuf;

use crate::error::PackError;
use crate::gear::Gear;
use super::stasher::Stasher;
use super::fisher::Fisher;
use super::ditcher::Ditcher;

pub trait Pack<G> {
    fn fish(&self, name: &str) -> Option<G>;
    fn dump(&self) -> Result<Vec<G>, PackError>;
    fn stash(&self, item: G) -> Result<Option<G>, PackError>;
    fn ditch(&self, name: &str) -> bool;
}

pub struct GearPack<F, S, D>
where
    F: Fisher<Gear>,
    S: Stasher<Gear>,
    D: Ditcher<Gear>
{
    pub path: PathBuf,
    pub fisher: F,
    pub stasher: S,
    pub ditcher: D
}

impl<F, S, D> GearPack<F, S, D>
where
    F: Fisher<Gear>,
    S: Stasher<Gear>,
    D: Ditcher<Gear>
{
    pub fn new(path: PathBuf, fisher: F, stasher: S, ditcher: D) -> Self {
        Self {
            path,
            fisher,
            stasher,
            ditcher
        }
    }
}

impl<F, S, D> Pack<Gear> for GearPack<F, S, D>
where
    F: Fisher<Gear>,
    S: Stasher<Gear>,
    D: Ditcher<Gear>
{
    fn fish(&self, name: &str) -> Option<Gear> {
        self.fisher.fish(&self.path, name)
    }

    fn stash(&self, item: Gear) -> Result<Option<Gear>, PackError> {
        self.stasher.stash(&self.path, item)
    }

    fn dump(&self) -> Result<Vec<Gear>, PackError> {
        self.fisher.dump(&self.path)
    }

    fn ditch(&self, name: &str) -> bool {
        self.ditcher.ditch(&self.path, name)
    }
}
