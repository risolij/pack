use std::path::PathBuf;

use crate::error::PackError;
use crate::gear::{Gear, Fishable, Stashable};
use super::stasher::Stasher;
use super::fisher::Fisher;

pub trait Pack {
    type Gear: Fishable + Stashable;

    fn fish(&self, name: &str) -> Option<Self::Gear>;
    fn dump(&self) -> Result<Vec<Self::Gear>, PackError>;
    fn stash(&self, item: Self::Gear) -> Result<Option<Self::Gear>, PackError>;
}

pub struct GearPack<F, S>
where
    F: Fisher,
    S: Stasher
{
    pub path: PathBuf,
    pub fisher: F,
    pub stasher: S
}

impl<F, S> GearPack<F, S>
where
    F: Fisher,
    S: Stasher
{
    pub fn new(path: PathBuf, fisher: F, stasher: S) -> Self {
        Self {
            path,
            fisher,
            stasher
        }
    }
}

impl<F, S> Pack for GearPack<F, S>
where
    F: Fisher<Gear = Gear>,
    S: Stasher<Gear = Gear>
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
}
