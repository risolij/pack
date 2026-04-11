use std::collections::HashMap;
use std::path::Path;
use std::fs;
use uuid::Uuid;

#[derive(Debug)]
pub struct Pocket {
    items: HashMap<Uuid, Item>
}

impl Pocket {
    pub fn reach() -> Self {
        let pocket_local = format!("{}/.local/share/mark", std::env!("HOME"));
        let pocket_path = Path::new(&pocket_local);

        if !pocket_path.exists() {
            fs::create_dir_all(pocket_path).unwrap();
            return Self { items: HashMap::new() }
        } 

        let mut items: HashMap<Uuid, Item> = HashMap::new();

        fs::read_dir(pocket_path)
            .unwrap()
            .map(|path| path.unwrap().path())
            .for_each(|path| {
                let contents = fs::read_to_string(path).unwrap();
                let id = Uuid::new_v4();
                items.insert(id, contents.into());
            });

        Self { items: items }
    }

    pub fn keeps(&mut self, item: impl Into<Item>) {
        self.items.insert(Uuid::new_v4(), item.into());
    }

}

trait Pocketable {}

#[derive(Debug)]
struct Item(String);

impl From<String> for Item {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Pocketable for Item { }
