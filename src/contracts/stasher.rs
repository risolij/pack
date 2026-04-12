use crate::models::gear::Gear;

pub trait Stasher {
    fn stash(&self, gear: Gear);
}
