mod cli;
mod error;
mod gear;
mod util;
mod storage;

use cli::*;
use error::PackError;
use storage::*;
use util::loadout;
use crate::storage::pack::Pack;

use clap::Parser;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), PackError> {
    let path = loadout()?;
    let pack = GearPack::new(
        path,
        GearFisher::new(),
        GearStasher::new()
    );

    let cli = Cli::parse();

    match cli.actions {
        Actions::Dump => {
            pack.dump()?
                .iter()
                .for_each(|gear| println!("{}", gear.name()));
        },
        Actions::Fish { name } => {
            if let Some(gear) = pack.fish(&name) {
                println!("{}", gear);
            } else {
                println!("No gear found matching '{}'", &name);
            }
        }
        Actions::Stash(proto) => {
            if let Ok(gear) = proto.try_into() {
                if let Some(ok_gear) = pack.stash(gear)? {
                    println!("{}", ok_gear);
                }
            }
        }
    }
    
    Ok(())
}
