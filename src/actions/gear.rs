use std::path::PathBuf;
use crate::contracts::pocketable::Pocketable;

pub enum Gear {
    File(GearFile),
    Text(GearText)
}
impl Pocketable for Gear {
    fn into_bytes(&self) -> Vec<u8> {
        match self {
            Self::File(file) => {
                file.into_bytes()
            },
            Self::Text(text) => {
                text.into_bytes()
            }
        }
    }

    fn name(&self) -> String {
        match self {
            Self::File(file) => file.name().to_owned(),
            Self::Text(text) => text.name().to_owned()
        }
    }
}

pub struct GearFile {
    pub name: String,
    pub file: PathBuf
}


impl Pocketable for GearFile {
    fn into_bytes(&self) -> Vec<u8> {
        std::fs::read("abc").unwrap()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }
}

impl GearFile {
    pub fn new(name: String, file: PathBuf) -> Self {
        Self { name, file }
    }
}

pub struct GearText {
    pub name: String,
    pub contents: String
}

impl Pocketable for GearText {
    fn into_bytes(&self) -> Vec<u8> {
        self.contents.to_owned().into_bytes()
    }

    fn name(&self) -> String {
        self.name.to_owned()
    }
}

impl GearText {
    pub fn new(name: String, contents: String) -> Self {
        Self { name, contents }
    }
}
