use anyhow::Result;
use clap::{ Parser };
use crate::actions::cmd::{ Cli, Commands };
use crate::models::pocket::Pocket;

mod actions;
mod models;

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
        Commands::Stash( stash) => pocket.stash(stash.into()),
        Commands::Reach( reach ) => pocket.reach(&reach.name),
    };

    Ok(())
}
