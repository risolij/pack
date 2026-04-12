use crate::actions::gear::*;

pub trait Pocketable {
    fn into_bytes(&self) -> Vec<u8>;
    fn name(&self) -> String;
}
