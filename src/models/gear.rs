use std::path::PathBuf;

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
