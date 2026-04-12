use clap::{ Subcommand, Parser };
use super::{
    reach::Reach,
    stash::Stash
};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Dump,
    #[command(subcommand)]
    Reach(Reach),
    Stash(Stash)
}
