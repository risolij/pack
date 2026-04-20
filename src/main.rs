mod cli;
mod error;
mod gear;
mod util;
mod storage;

use cli::*;
use error::PackError;
use storage::*;
use util::loadout;
use crate::gear::Highlightable;
use crate::storage::pack::Pack;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

use clap::Parser;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), PackError> {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let path = loadout()?;
    let pack = GearPack::new(
        path,
        GearFisher::new(),
        GearStasher::new(),
        GearDitcher::new()
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
                gear.highlight(&ss, &ts);
            } else {
                println!("No gear found matching '{}'", &name);
            }
        }
        Actions::Stash(proto) => {
            if let Ok(gear) = proto.try_into() {
                if let Some(ok_gear) = pack.stash(gear)? {
                    ok_gear.highlight(&ss, &ts);
                }
            }
        }
        Actions::Ditch { name } => {
            match pack.ditch(&name) {
                true => println!("Gear '{}' ditched", &name),
                false => println!("No gear found matching '{}'", &name),
            }
        }
    }
    
    Ok(())
}
