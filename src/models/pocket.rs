use super::gear::*;
use crate::contracts::{
    reacher::Reacher,
    stasher::Stasher
};

#[derive(Debug, Clone)]
pub struct Pocket<R, S>
where
    R: Reacher,
    S: Stasher
{
    reacher: R,
    stasher: S
}

impl<R,S> Pocket<R, S>
where
    R: Reacher,
    S: Stasher
{
    pub fn pack(reacher: R, stasher: S) -> Self {
        Self { 
            reacher,
            stasher,
        }
    }

    pub fn reach(&self, name: &str) {
        self.reacher.reach(name);
    }

    pub fn stash(&self, gear: Gear) {
        self.stasher.stash(gear);
    }

    pub fn dump(&self) {
        self.reacher.dump();
    }
}
