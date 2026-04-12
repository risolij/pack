use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Result;
use clap::{ Parser };
use crate::actions::cmd::{ Cli, Commands };
use crate::models::pocket::Pocket;
use crate::models::gear::{ GearReacher, GearStasher };

mod actions;
mod models;
mod contracts;

fn main() {
    if run().is_err() {
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let home = std::env::var("HOME").unwrap();
    let local = format!("{}/.local/share/pocket", home);

    let reacher = GearReacher::pack(PathBuf::from_str(&local)?);
    let stasher = GearStasher::pack(PathBuf::from_str(&local)?);
    let pocket = Pocket::pack(reacher, stasher);

    match cli.commands {
        Commands::Dump => pocket.dump(),
        Commands::Stash(stash) => pocket.stash(stash.into()),
        Commands::Reach(reach) => pocket.reach(&reach.name),
    };

    Ok(())
}
