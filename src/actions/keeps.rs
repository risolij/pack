use clap::{ Args, ArgGroup};
use super::pocket::Pocket;

#[derive(Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .multiple(false)
        .args(["file", "text"])
))]
pub struct Keeps {
    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    text: Option<String>
}

impl Keeps {
    pub fn run(&self, mut pocket: Pocket) {
        if self.file.is_some() {
            pocket.keeps(self.file.as_ref().unwrap().clone())
        }

        if self.text.is_some() {
            pocket.keeps(self.text.as_ref().unwrap().clone())
        }
    }
}


