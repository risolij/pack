use clap::{Subcommand, ValueEnum};

#[derive(Subcommand, Clone, Debug)]
pub enum Reach {
    Scaffold {
        #[arg(value_enum)]
        kind: ScaffoldKind
    },
    Snippet {
        name: String
    }
}

#[derive(ValueEnum, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
pub enum ScaffoldKind {
    Html,
    Css
}
