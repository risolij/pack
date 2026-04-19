use std::fs;
use std::path::PathBuf;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Style};
use syntect::util::{ as_24_bit_terminal_escaped, LinesWithEndings};

use crate::error::PackError;
use crate::cli::ProtoGear;
use crate::cli::Extension;

#[derive(Default, Debug)]
pub struct Gear {
    pub name: String,
    pub contents: Vec<u8>,
    pub extension: Extension
}

impl Gear {
    pub fn new(name: String, contents: Vec<u8>, extension: Extension) -> Self {
        Self { name, contents, extension }
    }

    pub fn from_file(name: String, file: PathBuf, extension: Extension) -> Result<Self, PackError> {

        Ok(Self {
            name,
            contents: fs::read(file)?,
            extension
        })
    }

    pub fn from_stdin(name: String, stdin: String, extension: Extension) -> Self {
        Self {
            name,
            contents: stdin.into_bytes(),
            extension
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.contents.clone()).unwrap_or_else(|_| "binary_data".to_string())
    }
}

impl TryFrom<ProtoGear> for Gear {
    type Error = PackError;

    fn try_from(value: ProtoGear) -> Result<Self, Self::Error> {
        match (value.file, value.stdin) {
            (Some(file), None) => {
                Gear::from_file(value.name, file, value.extension)
            },
            (None, Some(stdin)) => {
                Ok(Gear::from_stdin(value.name, stdin, value.extension))
            }
            _ => Err(PackError::ProtoGearConversion)
        }
    }
}

pub trait Fishable {}
pub trait Stashable {}
pub trait Ditchable {}
pub trait Painter {
    fn paint(&self, ss: &SyntaxSet, ts: &ThemeSet);
}

impl Ditchable for Gear {}
impl Fishable for Gear {}
impl Stashable for Gear {}

impl Painter for Gear {
    fn paint(&self, ss: &SyntaxSet, ts: &ThemeSet) {
        let syntax = ss.find_syntax_by_extension("rs").unwrap();
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        LinesWithEndings::from(&self.to_string())
            .into_iter()
            .for_each(|line| {
                let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ss).unwrap();
                let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                print!("{escaped}");
            });

        print!("\x1b[0m");
    }
}
