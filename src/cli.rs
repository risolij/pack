use clap::{ Subcommand, Parser, Args, ValueEnum };
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
    pub stdin: Option<String>,

    #[arg(value_enum, long, short)]
    pub extension: Extension,
}

#[derive(ValueEnum, Default, Debug, Clone, Eq, PartialEq)]
pub enum Extension {
    Rs,
    #[default]
    Txt,
    Md
}

impl Extension {
    pub fn as_str(&self) -> &str {
        match  self {
            Self::Rs => "rs",
            Self::Txt => "txt",
            Self::Md => "md"
        }
    }
}
