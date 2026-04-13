use std::env;
use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::error::PackError;

pub fn loadout() -> Result<PathBuf, PackError> {
    let home = env::var("HOME")?;
    let data_path = format!("{}/.local/share/pack", home);

    let path = PathBuf::from(data_path);

    if !path.exists() {
        create_dir_all(&path)?;
    }

    Ok(path)
}
