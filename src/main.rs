use anyhow::Result;
use crate::actions::*;
use clap::{ Parser };
use std::sync::{ Arc };

mod actions;

fn main() {
    if let Err(_) = run() {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}

fn run() -> Result<()> {
    let cli = cmd::Cli::parse();
    let pocket = Arc::new(pocket::Pocket::reach());

    match &cli.commands {
        cmd::Commands::Owns(owns) => owns.run(pocket.clone()),
        cmd::Commands::Keeps(keeps) => keeps.run(pocket.clone()),
        cmd::Commands::Wants { wants } => wants.run(),
    };

    Ok(())
}
