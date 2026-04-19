use clap::{ Subcommand, Parser, Args };
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub actions: Actions
}

#[derive(Subcommand)]
pub enum Actions {
    Dump,
    Fish { name: String },
    Stash(ProtoGear),
    Ditch { name: String }
}

#[derive(Args)]
pub struct ProtoGear {
    #[arg(long, short)]
    pub name: String,

    #[arg(long, short, group="input")]
    pub file: Option<PathBuf>,

    #[arg(long, short, group="input")]
    pub stdin: Option<String>
}

