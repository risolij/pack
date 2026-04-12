use clap::Args;
use std::path::PathBuf;
use super::gear::*;

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

impl Stash {
    pub fn into_gear(self) -> Gear {
        if self.input.file.is_some() {
            return Gear::File(
                GearFile::new(self.name, self.input.file.unwrap())
            );
        } else {
            return Gear::Text(
                GearText::new(self.name, self.input.text.unwrap())
            );
        }
    }
}
