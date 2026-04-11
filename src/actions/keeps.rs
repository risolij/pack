use clap::{ Args, ArgGroup};
use super::pocket::Pocket;
use super::edc::Edc;
use std::sync::Arc;

#[derive(Args)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .multiple(false)
        .args(["file", "text"])
))]
pub struct Keeps {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
    file: Option<String>,

    #[arg(short, long)]
    text: Option<String>
}

impl Keeps {
    pub fn run(&self, pocket: Arc<Pocket>) {
        if self.file.is_some() {
            let edc = Edc::new(self.name.to_owned(), self.file.to_owned().unwrap());
            pocket.keeps_file(edc);
        } else {
            let text = self.text.to_owned().unwrap();
            let edc = Edc::new(self.name.to_owned(), text);
            pocket.keeps_text(edc);
        }
    }
}


