use clap::{ Subcommand, Parser };
use super::{
    owns::Owns,
    wants::Wants,
    keeps::Keeps
};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "List of marks items")]
    Owns(Owns),
    Wants {
        #[arg(value_enum)]
        wants: Wants, 
    },
    Keeps(Keeps)
}


