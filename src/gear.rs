use std::fs;
use std::path::PathBuf;
use syntect::easy::HighlightLines;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

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
    pub fn new(name: impl Into<String>, contents: impl Into<Vec<u8>>, extension: Extension) -> Self {
        Self {
            name: name.into(),
            contents: contents.into(),
            extension
        }
    }

    pub fn from_file(name: impl Into<String>, file: impl Into<PathBuf>, extension: Extension) -> Result<Self, PackError> {

        Ok(Self {
            name: name.into(),
            contents: fs::read(file.into())?,
            extension
        })
    }

    pub fn from_stdin(name: impl Into<String>, stdin: impl Into<String>, extension: Extension) -> Self {
        Self::new(name, stdin.into(), extension)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn to_string(&self) -> Result<&str, PackError> {
        std::str::from_utf8(&self.contents)
            .map_err(|_| PackError::GearUtf8Error)
    }
}

impl TryFrom<ProtoGear> for Gear {
    type Error = PackError;

    fn try_from(value: ProtoGear) -> Result<Self, Self::Error> {
        match (value.file, value.stdin) {
            (Some(file), None) => Gear::from_file(value.name, file, value.extension),
            (None, Some(stdin)) => Ok(Gear::from_stdin(value.name, stdin, value.extension)),
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
        let syntax = ss
            .find_syntax_by_extension(self.extension.as_str())
            .unwrap_or(ss.find_syntax_plain_text());
            
        let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

        let Ok(data) = self.to_string() else {
            return;
        };

        LinesWithEndings::from(data)
            .into_iter()
            .for_each(|line| {
                if let Ok(ranges) = h.highlight_line(line, &ss) {
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                    print!("{escaped}");
                }
            });

        print!("\x1b[0m");
    }
}
