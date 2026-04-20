use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::error::PackError;
use std::str::FromStr;

enum PackSystem {
    Windows,
    Linux,
    Mac
}

impl PackSystem {
    fn local_share(&self) -> Result<String, PackError> {
        let pack_store = match self {
            PackSystem::Linux => {
                let home = env::var("HOME")?;
                format!("{}/.local/share/pack", home)
            },
            PackSystem::Windows => {
                let home = env::var("USERPROFILE")?;
                format!("{}\\AppData\\Local\\pack", home)
            },
            PackSystem::Mac => {
                let home = env::var("HOME")?;
                format!("{}/Library/Application Support/pack", home)
            },
        };

        Ok(pack_store)
    }
}

impl FromStr for PackSystem {
    type Err = PackError;

    fn from_str(os: &str) -> Result<Self, PackError> {
        match os {
            "linux" => Ok(Self::Linux),
            "windows" => Ok(Self::Windows),
            "macos" => Ok(Self::Mac),
            _ => Err(PackError::PackSystemUnkown)
        }
    }
}

pub fn loadout() -> Result<PathBuf, PackError> {
    let os = env::consts::OS;
    let pack_system = PackSystem::from_str(os)?.local_share()?;

    let path = PathBuf::from(pack_system);

    if !path.exists() {
        create_dir_all(&path)?;
    }

    Ok(path)
}
