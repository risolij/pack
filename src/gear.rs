use std::fs;
use std::path::PathBuf;
use crate::error::PackError;
use crate::cli::ProtoGear;

#[derive(Default, Debug)]
pub struct Gear {
    pub name: String,
    pub contents: Vec<u8>
}

impl Gear {
    pub fn new(name: String, contents: Vec<u8>) -> Self {
        Self { name, contents }
    }

    pub fn from_file(name: String, file: PathBuf) -> Result<Self, PackError> {
        Ok(Self {
            name,
            contents: fs::read(file)?
        })
    }

    pub fn from_stdin(name: String, stdin: String) -> Self {
        Self {
            name,
            contents: stdin.into_bytes()
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl TryFrom<ProtoGear> for Gear {
    type Error = PackError;

    fn try_from(value: ProtoGear) -> Result<Self, Self::Error> {
        match (value.file, value.stdin) {
            (Some(file), None) => {
                Gear::from_file(value.name, file)
            },
            (None, Some(stdin)) => {
                Ok(Gear::from_stdin(value.name, stdin))
            }
            _ => {
                eprintln!("Something went wrong");
                std::process::exit(1);
            }
        }
    }
}

impl std::fmt::Display for Gear {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = String::from_utf8(self.contents.to_owned())
            .unwrap_or_else(|_| "binary_data".to_string());

        write!(f, "{}\n--------------------------\n{}", self.name, text)
    }
}

pub trait Fishable {}
pub trait Stashable {}

impl Fishable for Gear {}
impl Stashable for Gear {}
