use std::path::PathBuf;

use crate::error::PackError;
use crate::gear::Gear;
use super::stasher::Stasher;
use super::fisher::Fisher;
use super::ditcher::Ditcher;

pub trait Pack {
    type Gear;

    fn fish(&self, name: &str) -> Option<Self::Gear>;
    fn dump(&self) -> Result<Vec<Self::Gear>, PackError>;
    fn stash(&self, item: Self::Gear) -> Result<Option<Self::Gear>, PackError>;
    fn ditch(&self, name: &str) -> bool;
}

pub struct GearPack<F, S, D>
where
    F: Fisher<Gear>,
    S: Stasher<Gear>,
    D: Ditcher
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
    D: Ditcher
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

impl<F, S, D> Pack for GearPack<F, S, D>
where
    F: Fisher<Gear>,
    S: Stasher<Gear>,
    D: Ditcher
{
    type Gear = Gear;

    fn fish(&self, name: &str) -> Option<Self::Gear> {
        self.fisher.fish(&self.path, name)
    }

    fn stash(&self, item: Self::Gear) -> Result<Option<Self::Gear>, PackError> {
        self.stasher.stash(&self.path, item)
    }

    fn dump(&self) -> Result<Vec<Self::Gear>, PackError> {
        self.fisher.dump(&self.path)
    }

    fn ditch(&self, name: &str) -> bool {
        self.ditcher.ditch(&self.path, name)
    }
}
