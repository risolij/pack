use clap::ValueEnum;

#[derive(ValueEnum, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Debug)]
pub enum Wants {
    Html
}

impl Wants {
    pub fn run(&self) {
        println!("Running: {:?}", self);
    }
}

