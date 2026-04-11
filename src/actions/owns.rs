use clap::Args;

#[derive(Args)]
pub struct Owns;

impl Owns {
    pub fn run(&self) {
        println!("mark owns");
    }
}
