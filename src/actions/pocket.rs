use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use super::edc::Edc;

#[derive(Debug, Clone)]
pub struct Pocket {
    path: PathBuf
}

impl Pocket {
    pub fn reach() -> Self {
        let pocket_local = format!("{}/.local/share/mark", std::env!("HOME"));
        let pocket_path = Path::new(&pocket_local);

        if !pocket_path.exists() {
            fs::create_dir_all(pocket_path).unwrap();
        } 

        Self { path: pocket_path.to_path_buf() }
    }

    pub fn owns(&self) {
        let Ok(dirs) = fs::read_dir(&self.path) else {
            return;
        };

        dirs
            .into_iter()
            .filter_map(|path| path.ok())
            .map(|path| path.file_name())
            .for_each(|path| println!("{}", path.display()));
    }

    fn keeps(&self, edc: &Edc) -> Result<File, std::io::Error> {
        File::create_new(self.path.join(edc.to_path()))
    }

    pub fn keeps_file(&self, edc: Edc) {
        let Ok(mut store) = self.keeps(&edc) else {
            return;
        };

        let file = edc.to_contents();
        let contents = String::from_utf8(std::fs::read(file).unwrap()).unwrap();

        store.write_all(contents.as_bytes()).unwrap();

    }

    pub fn keeps_text(&self, edc: Edc) {
        let Ok(mut file) = self.keeps(&edc) else {
            return;
        };

        file.write_all(edc.to_contents().as_bytes()).unwrap();
    }
}
