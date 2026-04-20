use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::error::PackError;
use crate::gear::{Stashable, Gear};

pub trait Stasher<G>
where
    G: Stashable
{
    fn stash(&self, path: &PathBuf, item: G) -> Result<Option<G>, PackError>;
}

pub struct GearStasher;

impl GearStasher {
    pub fn new() -> Self {
        Self { }
    }
}

impl Stasher<Gear> for GearStasher {
    fn stash(&self, path: &PathBuf, item: Gear) -> Result<Option<Gear>, PackError> {
        let path = path.join(&item.name);
        let mut file = File::create_new(path)?;

        file.write_all(&item.contents)?;

        Ok(Some(item))
    }
}
