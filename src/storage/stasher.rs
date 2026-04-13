use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::error::PackError;
use crate::gear::{Gear, Stashable};

pub trait Stasher {
    type Gear: Stashable;

    fn stash(&self, path: &PathBuf, item: Self::Gear) -> Result<Option<Self::Gear>, PackError>;
}

pub struct GearStasher;

impl GearStasher {
    pub fn new() -> Self {
        Self { }
    }
}

impl Stasher for GearStasher {
    type Gear = Gear;

    fn stash(&self, path: &PathBuf, item: Self::Gear) -> Result<Option<Self::Gear>, PackError> {
        let path = path.join(&item.name);
        let mut file = File::create_new(path)?;

        file.write_all(&item.contents)?;

        Ok(Some(item))
    }
}
