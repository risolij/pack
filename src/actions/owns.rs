use clap::Args;
use super::pocket::Pocket;
use std::sync::{ Arc };

#[derive(Args)]
pub struct Owns;

impl Owns {
    pub fn run(&self, pocket: Arc<Pocket>) {
        pocket.owns();
    }
}
