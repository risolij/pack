use std::fs::{File, create_dir_all, read, read_dir};
use std::io::Write;
use std::path::{ Path, PathBuf };
use crate::contracts::{
    reacher::Reacher,
    stasher::Stasher
};

pub struct Gear {
    pub name: String,
    pub gear_type: GearType
}

impl Gear {
    pub fn new(name: String, gear_type: GearType) -> Self {
        Self {
            name,
            gear_type
        }
    }
}

pub enum GearType {
    File(PathBuf),
    Text(String)
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

    fn reach_local(&self, name: PathBuf) -> Vec<u8> {
        read(name).unwrap()
    }

    fn dump(&self) {
        let Ok(dirs) = read_dir(&self.path) else {
            return;
        };

        dirs
            .into_iter()
            .filter_map(|path| path.ok())
            .map(|path| path.file_name())
            .for_each(|path| println!("{}", path.display()));
    }
}

impl Stasher for GearStasher {
    fn stash(&self, gear: Gear) {
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
