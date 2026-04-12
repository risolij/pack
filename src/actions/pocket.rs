use std::{
    fs::create_dir_all,
    fs::read_dir,
    fs::File,
    fs::read
};
use std::io::Write;
use std::path::{Path, PathBuf};
use crate::actions::{
    reach::*,
    gear::*
};
use crate::contracts::pocketable::Pocketable;

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

    pub fn search(&self, name: &str) {
        let reader = read(self.path.join(name)).unwrap();
        println!("{}", String::from_utf8(reader).unwrap());
    }

    pub fn reach(&self, gear: Reach) {
        match gear {
            Reach::Snippet { name } => {
                self.search(&name);
            },
            Reach::Scaffold { kind } => {
                todo!()
            },
        }
    }

    pub fn stash(&self, gear: Gear) {
        let contents = gear.into_bytes();
        let name = gear.name();
        let path = self.path.join(name);

        let mut file = File::create_new(path)
            .expect("Failed to create file");

        file.write_all(&contents);
    }
}
