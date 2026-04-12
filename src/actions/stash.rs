use clap::Args;
use std::path::PathBuf;

#[derive(Args, Clone)]
pub struct Stash {
    #[arg(short, long)]
    pub name: String,

    #[command(flatten)]
    pub input: StashInput
}

#[derive(Args, Clone)]
#[group(required = true, multiple = false)]
pub struct StashInput {
    #[arg(short, long)]
    pub file: Option<PathBuf>,

    #[arg(short, long)]
    pub text: Option<String>
}
