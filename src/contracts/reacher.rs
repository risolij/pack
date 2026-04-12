use std::path::PathBuf;

pub trait Reacher {
    fn reach(&self, name: &str);
    fn reach_local(&self, name: PathBuf) -> Vec<u8>;
    fn dump(&self);
}
