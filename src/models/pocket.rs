use std::{
    fs::create_dir_all,
    fs::read_dir,
    fs::File,
    fs::read
};
use std::io::Write;
use std::path::{Path, PathBuf};
use super::gear::*;

#[derive(Debug, Clone)]
pub struct Pocket {
    path: PathBuf
}

impl Pocket {
    pub fn pack() -> Self {
        let base_env = std::env::var("HOME").expect("HOME not set");
        let pocket_local = format!("{}/.local/share/pocket", base_env);
        let pocket_path = Path::new(&pocket_local);

        if !pocket_path.exists() {
            create_dir_all(pocket_path).unwrap();
        } 

        Self { path: pocket_path.to_path_buf() }
    }

    pub fn dump(&self) {
        let Ok(dirs) = read_dir(&self.path) else {
            return;
        };

        dirs
            .into_iter()
            .filter_map(|path| path.ok())
            .map(|path| path.file_name())
            .for_each(|path| println!("{}", path.display()));
    }

    pub fn reach(&self, name: &str) {
        let bytes = read(self.path.join(name)).unwrap();
        let text = String::from_utf8(bytes).unwrap();

        println!("{}", text);
    }

    pub fn stash(&self, gear: Gear) {
        match gear.gear_type {
            GearType::File(file) => {
                let contents = read(file).unwrap();
                let name = gear.name;
                let mut file = File::create_new(self.path.join(name)).unwrap();
                file.write_all(&contents).unwrap();

            },
            GearType::Text(text) => {
                let contents = text.as_bytes();
                let name = gear.name;

                let mut file = File::create_new(self.path.join(name)).unwrap();

                file.write_all(&contents).unwrap();
            }
        }
    }
}
