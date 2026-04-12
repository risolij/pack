use std::fs::{File, create_dir_all, read, read_dir};
use std::io::Write;
use std::path::{ Path, PathBuf };
use crate::actions::stash::Stash;
use crate::contracts::{
    reacher::Reacher,
    stasher::Stasher
};

pub struct Gear {
    pub name: String,
    pub data: Vec<u8>
}

impl Gear {
    pub fn new(name: String, data: Vec<u8>) -> Self {
        Self {
            name,
            data
        }
    }
}

impl From<Stash> for Gear {
    fn from(stash: Stash) -> Self {
        let data = if let Some(path) = stash.input.file {
            read(path).unwrap()
        } else {
            stash.input.text.unwrap().into_bytes()
        };

        Gear::new(stash.name, data)
    }
}

pub struct GearReacher {
    path: PathBuf
}

impl GearReacher {
    pub fn pack(path: PathBuf) -> Self {
        let base_env = std::env::var("HOME").expect("HOME not set");
        let pocket_local = format!("{}/.local/share/pocket", base_env);
        let pocket_path = Path::new(&pocket_local);

        if !pocket_path.exists() {
            create_dir_all(pocket_path).unwrap();
        } 

        Self { path }
    }
}

pub struct GearStasher {
    path: PathBuf
}

impl GearStasher {
    pub fn pack(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Reacher for GearReacher {
    fn reach(&self, name: &str) {
        let bytes = read(self.path.join(name)).unwrap();
        let text = String::from_utf8(bytes).unwrap();

        println!("{}", text);
    }

    fn dump(&self) {
        let Ok(dirs) = read_dir(&self.path) else {
            return;
        };

        dirs
            .flatten()
            .map(|path| path.file_name().to_string_lossy().to_string())
            .for_each(|path| println!("{}", path));
    }
}

impl Stasher for GearStasher {
    fn stash(&self, gear: Gear) {
        let mut file = File::create_new(self.path.join(gear.name)).unwrap();
        file.write_all(&gear.data).unwrap();
    }
}
