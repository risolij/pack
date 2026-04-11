use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Edc {
    name: String,
    contents: String
}

impl Edc {
    pub fn new(name: String, contents: String) -> Self {
        Self {
            name,
            contents
        }
    }

    pub fn to_path(&self) -> PathBuf {
        PathBuf::from(&self.name)
    }

    pub fn to_contents(&self) -> &str {
        &self.contents
    }
}
