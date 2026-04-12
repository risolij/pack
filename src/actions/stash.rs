use clap::Args;
use std::path::PathBuf;
use crate::models::gear::*;

#[derive(Args, Clone)]
pub struct Stash {
    #[arg(short, long)]
    name: String,

    #[command(flatten)]
    input: StashInput
}

#[derive(Args, Clone)]
#[group(required = true, multiple = false)]
pub struct StashInput {
    #[arg(short, long)]
    file: Option<PathBuf>,

    #[arg(short, long)]
    text: Option<String>
}

impl From<Stash> for Gear {
    fn from(stash: Stash) -> Gear {
        if stash.input.file.is_some() {
            Gear::new(stash.name, GearType::File(stash.input.file.unwrap()))
        } else {
            Gear::new(stash.name, GearType::Text(stash.input.text.unwrap()))
        }
    }
}
