use anyhow::Result;
use clap::{ Parser };
use crate::actions::{
    cmd::Commands,
    cmd::Cli,
    pocket::Pocket
};

mod contracts;
mod actions;

fn main() {
    if run().is_err() {
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let pocket = Pocket::pack();

    match cli.commands {
        Commands::Dump => pocket.dump(),
        Commands::Stash( gear ) => pocket.stash(gear.into_gear()),
        Commands::Reach( gear ) => pocket.reach(gear),
    };

    Ok(())
}
