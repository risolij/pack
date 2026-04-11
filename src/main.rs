use anyhow::Result;
use crate::actions::*;
use clap::{ Parser };

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
    let pocket = pocket::Pocket::reach();

    match &cli.commands {
        cmd::Commands::Owns(owns) => owns.run(),
        cmd::Commands::Keeps(keeps) => keeps.run(pocket),
        cmd::Commands::Wants { wants } => wants.run(),
    };

    Ok(())
}
